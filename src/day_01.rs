use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let contents = get_file_contents("inputs/day_01.txt")?;
    let nums: Vec<i32> = contents.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    Ok(nums)
}

fn get_file_contents(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents)
}

fn count_num_increases(nums: &Vec<i32>, interval_len: usize) -> i32 {
    let nums_len = nums.len();
    let mut increase_count: i32 = 0;
    let mut previous: i32 = 0;
    for i in 0..interval_len {
        previous += &nums[i];
    }
    for i in 1..=(nums_len - interval_len) {
        let current = previous - &nums[i - 1] + &nums[i + interval_len - 1];
        if current > previous {
            increase_count += 1;
        }
        previous = current;
    }
    increase_count
}

fn main() -> std::io::Result<()> {
    let nums: Vec<i32> = parse_input()?;
    println!("Part 1: # of increases: {}", count_num_increases(&nums, 1));
    println!("Part 2: # of interval increases: {}", count_num_increases(&nums, 3));
    Ok(())
}

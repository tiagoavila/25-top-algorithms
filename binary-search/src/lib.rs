pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: usize = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid: usize = left + (right - left) / 2; // avoid overflow
        
        if nums[mid] == target {
            return mid as i32;
        } 
        
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_finds_number_in_array() {
        let nums: [i32; 5] = [2, 3, 5, 7, 9];
        let target: i32 = 7;
        let result = binary_search(nums.to_vec(), target);
        assert_eq!(result, 3);
    }

    #[test]
    fn binary_search_returns_minus_one_when_number_not_in_array() {
        let nums: [i32; 5] = [2, 3, 5, 7, 9];
        let target: i32 = 8;
        let result = binary_search(nums.to_vec(), target);
        assert_eq!(result, -1);
    }
}

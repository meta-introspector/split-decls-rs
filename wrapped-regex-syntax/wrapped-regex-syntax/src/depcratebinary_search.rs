// Generated macro for binary_search (function)
macro_rules! Depcratebinary_search {
() => {
// Module: crate
// Provides: {"binary_search"}
// Dependencies: {}
# [doc = " Binary search to find first element such that `pred(T) == true`."] # [doc = ""] # [doc = " Assumes that if `pred(xs[i]) == true` then `pred(xs[i+1]) == true`."] # [doc = ""] # [doc = " If all elements yield `pred(T) == false`, then `xs.len()` is returned."] fn binary_search < T , F > (xs : & [T] , mut pred : F) -> usize where F : FnMut (& T) -> bool { let (mut left , mut right) = (0 , xs . len ()) ; while left < right { let mid = (left + right) / 2 ; if pred (& xs [mid]) { right = mid ; } else { left = mid + 1 ; } } left }
};
}

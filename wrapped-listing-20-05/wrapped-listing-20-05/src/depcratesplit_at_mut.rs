// Generated macro for split_at_mut (function)
macro_rules! Depcratesplit_at_mut {
() => {
// Module: crate
// Provides: {"split_at_mut"}
// Dependencies: {}
fn split_at_mut (values : & mut [i32] , mid : usize) -> (& mut [i32] , & mut [i32]) { let len = values . len () ; assert ! (mid <= len) ; (& mut values [.. mid] , & mut values [mid ..]) }
};
}

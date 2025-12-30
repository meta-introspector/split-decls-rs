// Generated macro for count_with_decimals (function)
macro_rules! Depcrate_progresscount_with_decimals {
() => {
// Module: crate::progress
// Provides: {"count_with_decimals"}
// Dependencies: {}
# [doc = " A unit for displaying human readable numbers with `name` suffix,"] # [doc = " with throughput and progress percentage, and `decimals` decimal places."] # [cfg (not (feature = "progress-unit-human-numbers"))] pub fn count_with_decimals (name : & 'static str , _decimals : usize) -> Option < Unit > { Some (unit :: label_and_mode (name , unit :: display :: Mode :: with_throughput () . and_percentage () ,)) }
};
}

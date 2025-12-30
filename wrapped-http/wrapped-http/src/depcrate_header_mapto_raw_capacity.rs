// Generated macro for to_raw_capacity (function)
macro_rules! Depcrate_header_mapto_raw_capacity {
() => {
// Module: crate::header::map
// Provides: {"to_raw_capacity"}
// Dependencies: {}
# [inline] fn to_raw_capacity (n : usize) -> Result < usize , MaxSizeReached > { n . checked_add (n / 3) . ok_or_else (MaxSizeReached :: new) }
};
}

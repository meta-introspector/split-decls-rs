// Generated macro for in_inclusive_range_char (function)
macro_rules! Depcrate_uts46in_inclusive_range_char {
() => {
// Module: crate::uts46
// Provides: {"in_inclusive_range_char"}
// Dependencies: {}
# [inline (always)] fn in_inclusive_range_char (c : char , start : char , end : char) -> bool { u32 :: from (c) . wrapping_sub (u32 :: from (start)) <= (u32 :: from (end) - u32 :: from (start)) }
};
}

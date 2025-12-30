// Generated macro for in_inclusive_range (function)
macro_rules! Depcrate_elementsin_inclusive_range {
() => {
// Module: crate::elements
// Provides: {"in_inclusive_range"}
// Dependencies: {}
# [inline (always)] fn in_inclusive_range (c : char , start : char , end : char) -> bool { u32 :: from (c) . wrapping_sub (u32 :: from (start)) <= (u32 :: from (end) - u32 :: from (start)) }
};
}

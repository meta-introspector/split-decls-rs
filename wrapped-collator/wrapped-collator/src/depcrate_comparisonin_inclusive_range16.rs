// Generated macro for in_inclusive_range16 (function)
macro_rules! Depcrate_comparisonin_inclusive_range16 {
() => {
// Module: crate::comparison
// Provides: {"in_inclusive_range16"}
// Dependencies: {}
# [doc = " `true` iff `i` is greater or equal to `start` and less or equal"] # [doc = " to `end`."] # [inline (always)] fn in_inclusive_range16 (i : u16 , start : u16 , end : u16) -> bool { i . wrapping_sub (start) <= (end - start) }
};
}

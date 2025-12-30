// Generated macro for truncate (function)
macro_rules! Depcrate_utiltruncate {
() => {
// Module: crate::util
// Provides: {"truncate"}
// Dependencies: {}
# [doc = " Truncates `u64` and returns the lower 32 bits as `u32`"] pub (crate) fn truncate (val : u64) -> u32 { u32 :: try_from (val & u64 :: from (u32 :: MAX)) . expect ("The higher 32 bits are masked") }
};
}

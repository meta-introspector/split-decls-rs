// Generated macro for in_inclusive_range16 (function)
macro_rules! Depcratein_inclusive_range16 {
() => {
// Module: crate
// Provides: {"in_inclusive_range16"}
// Dependencies: {}
# [inline (always)] # [cfg (feature = "utf16_iter")] fn in_inclusive_range16 (u : u16 , start : u16 , end : u16) -> bool { u . wrapping_sub (start) <= (end - start) }
};
}

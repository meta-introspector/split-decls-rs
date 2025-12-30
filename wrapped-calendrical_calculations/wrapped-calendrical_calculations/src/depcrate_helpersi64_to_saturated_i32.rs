// Generated macro for i64_to_saturated_i32 (function)
macro_rules! Depcrate_helpersi64_to_saturated_i32 {
() => {
// Module: crate::helpers
// Provides: {"i64_to_saturated_i32"}
// Dependencies: {}
# [doc = " Convert an i64 to i32 but saturate at th ebounds"] # [inline] pub (crate) fn i64_to_saturated_i32 (input : i64) -> i32 { i64_to_i32 (input) . unwrap_or_else (| i | i . saturate ()) }
};
}

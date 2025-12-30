// Generated macro for safe_u64_to_f64 (function)
macro_rules! Depcrate_blobsafe_u64_to_f64 {
() => {
// Module: crate::blob
// Provides: {"safe_u64_to_f64"}
// Dependencies: {}
# [doc = " JavaScript only has `f64`, which has a maximum accurate integer size of`2^53 - 1`. So we use"] # [doc = " this to safely convert from larger integers to `f64`.  See"] # [doc = " [Number.MAX_SAFE_INTEGER](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER)"] fn safe_u64_to_f64 (number : u64) -> f64 { if number > (js_sys :: Number :: MAX_SAFE_INTEGER as u64) { throw_str ("a rust number was too large and could not be represented in JavaScript") ; } number as f64 }
};
}

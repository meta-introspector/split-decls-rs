// Generated macro for safe_f64_to_u64 (function)
macro_rules! Depcrate_blobsafe_f64_to_u64 {
() => {
// Module: crate::blob
// Provides: {"safe_f64_to_u64"}
// Dependencies: {}
# [doc = " Like safe_u64_to_f64, but additionally checks that the number is an integer."] fn safe_f64_to_u64 (number : f64) -> u64 { if number > js_sys :: Number :: MAX_SAFE_INTEGER { throw_str ("a rust number was too large and could not be represented in JavaScript") ; } if number . fract () != 0.0 { throw_str ("a number could not be converted to an integer because it was not a whole number" ,) ; } number as u64 }
};
}

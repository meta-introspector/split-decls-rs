// Generated macro for safe_u128_to_f64 (function)
macro_rules! Depcrate_blobsafe_u128_to_f64 {
() => {
// Module: crate::blob
// Provides: {"safe_u128_to_f64"}
// Dependencies: {}
fn safe_u128_to_f64 (number : u128) -> f64 { const MAX_SAFE_INTEGER : u128 = js_sys :: Number :: MAX_SAFE_INTEGER as u128 ; if number > MAX_SAFE_INTEGER { throw_str ("a rust number was too large and could not be represented in JavaScript") ; } number as f64 }
};
}

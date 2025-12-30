// Generated macro for normalize (function)
macro_rules! Depcrate_bellerophonnormalize {
() => {
// Module: crate::bellerophon
// Provides: {"normalize"}
// Dependencies: {}
# [doc = " Normalize float-point number."] # [doc = ""] # [doc = " Shift the mantissa so the number of leading zeros is 0, or the value"] # [doc = " itself is 0."] # [doc = ""] # [doc = " Get the number of bytes shifted."] pub fn normalize (fp : & mut ExtendedFloat) -> i32 { if fp . mant != 0 { let shift = fp . mant . leading_zeros () as i32 ; fp . mant <<= shift ; fp . exp -= shift ; shift } else { 0 } }
};
}

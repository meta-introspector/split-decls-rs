// Generated macro for impl_158 (impl)
macro_rules! Depcrate_floatimpl_158 {
() => {
// Module: crate::float
// Provides: {"impl_158"}
// Dependencies: {}
impl FloatLit < & str > { # [doc = " Makes a copy of the underlying buffer and returns the owned version of"] # [doc = " `Self`."] pub fn to_owned (& self) -> FloatLit < String > { FloatLit { raw : self . raw . to_owned () , end_integer_part : self . end_integer_part , end_fractional_part : self . end_fractional_part , end_number_part : self . end_number_part , } } }
};
}

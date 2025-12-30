// Generated macro for alt_bn128_multiplication (function)
macro_rules! Depcrate_multiplicationalt_bn128_multiplication {
() => {
// Module: crate::multiplication
// Provides: {"alt_bn128_multiplication"}
// Dependencies: {}
# [deprecated (since = "3.1.0" , note = "Please use `alt_bn128_g1_multiplication_be` instead")] # [inline (always)] pub fn alt_bn128_multiplication (input : & [u8]) -> Result < Vec < u8 > , AltBn128Error > { alt_bn128_g1_multiplication_be (input) }
};
}

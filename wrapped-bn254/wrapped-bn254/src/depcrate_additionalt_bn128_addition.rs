// Generated macro for alt_bn128_addition (function)
macro_rules! Depcrate_additionalt_bn128_addition {
() => {
// Module: crate::addition
// Provides: {"alt_bn128_addition"}
// Dependencies: {}
# [deprecated (since = "3.1.0" , note = "Please use `alt_bn128_g1_addition_be` instead")] # [inline (always)] pub fn alt_bn128_addition (input : & [u8]) -> Result < Vec < u8 > , AltBn128Error > { alt_bn128_g1_addition_be (input) }
};
}

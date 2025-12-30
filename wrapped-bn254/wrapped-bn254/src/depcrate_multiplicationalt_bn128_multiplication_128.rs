// Generated macro for alt_bn128_multiplication_128 (function)
macro_rules! Depcrate_multiplicationalt_bn128_multiplication_128 {
() => {
// Module: crate::multiplication
// Provides: {"alt_bn128_multiplication_128"}
// Dependencies: {}
# [deprecated (since = "3.1.0" , note = "Please use `alt_bn128_g1_multiplication_be` instead")] # [cfg (not (target_os = "solana"))] # [inline (always)] pub fn alt_bn128_multiplication_128 (input : & [u8]) -> Result < Vec < u8 > , AltBn128Error > { alt_bn128_versioned_g1_multiplication (VersionedG1Multiplication :: V0 , input , Endianness :: BE) }
};
}

// Generated macro for alt_bn128_g1_multiplication_be (function)
macro_rules! Depcrate_multiplicationalt_bn128_g1_multiplication_be {
() => {
// Module: crate::multiplication
// Provides: {"alt_bn128_g1_multiplication_be"}
// Dependencies: {}
# [inline (always)] pub fn alt_bn128_g1_multiplication_be (input : & [u8]) -> Result < Vec < u8 > , AltBn128Error > { # [cfg (not (target_os = "solana"))] { alt_bn128_versioned_g1_multiplication (VersionedG1Multiplication :: V1 , input , Endianness :: BE) } # [cfg (target_os = "solana")] { if input . len () > ALT_BN128_G1_MULTIPLICATION_INPUT_SIZE { return Err (AltBn128Error :: InvalidInputData) ; } let mut result_buffer = Vec :: with_capacity (ALT_BN128_G1_MULTIPLICATION_OUTPUT_SIZE) ; unsafe { let result = syscalls :: sol_alt_bn128_group_op (ALT_BN128_G1_MUL_BE , input as * const _ as * const u8 , input . len () as u64 , result_buffer . as_mut_ptr () ,) ; match result { 0 => { result_buffer . set_len (ALT_BN128_G1_MULTIPLICATION_OUTPUT_SIZE) ; Ok (result_buffer) } _ => Err (AltBn128Error :: UnexpectedError) , } } } }
};
}

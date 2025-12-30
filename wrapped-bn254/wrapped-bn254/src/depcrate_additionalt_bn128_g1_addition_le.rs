// Generated macro for alt_bn128_g1_addition_le (function)
macro_rules! Depcrate_additionalt_bn128_g1_addition_le {
() => {
// Module: crate::addition
// Provides: {"alt_bn128_g1_addition_le"}
// Dependencies: {}
# [inline (always)] pub fn alt_bn128_g1_addition_le (input : & [u8 ; ALT_BN128_G1_ADDITION_INPUT_SIZE] ,) -> Result < Vec < u8 > , AltBn128Error > { # [cfg (not (target_os = "solana"))] { alt_bn128_versioned_g1_addition (VersionedG1Addition :: V0 , input , Endianness :: LE) } # [cfg (target_os = "solana")] { let mut result_buffer = Vec :: with_capacity (ALT_BN128_G1_ADDITION_OUTPUT_SIZE) ; unsafe { let result = syscalls :: sol_alt_bn128_group_op (ALT_BN128_G1_ADD_LE , input as * const _ as * const u8 , input . len () as u64 , result_buffer . as_mut_ptr () ,) ; match result { 0 => { result_buffer . set_len (ALT_BN128_G1_ADDITION_OUTPUT_SIZE) ; Ok (result_buffer) } _ => Err (AltBn128Error :: UnexpectedError) , } } } }
};
}

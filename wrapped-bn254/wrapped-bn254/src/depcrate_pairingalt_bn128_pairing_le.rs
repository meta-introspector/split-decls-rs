// Generated macro for alt_bn128_pairing_le (function)
macro_rules! Depcrate_pairingalt_bn128_pairing_le {
() => {
// Module: crate::pairing
// Provides: {"alt_bn128_pairing_le"}
// Dependencies: {}
# [inline (always)] pub fn alt_bn128_pairing_le (input : & [u8]) -> Result < Vec < u8 > , AltBn128Error > { # [cfg (not (target_os = "solana"))] { alt_bn128_versioned_pairing (VersionedPairing :: V1 , input , Endianness :: LE) } # [cfg (target_os = "solana")] { if input . len () % ALT_BN128_PAIRING_ELEMENT_SIZE != 0 { return Err (AltBn128Error :: InvalidInputData) ; } let mut result_buffer = Vec :: with_capacity (ALT_BN128_PAIRING_OUTPUT_SIZE) ; unsafe { let result = syscalls :: sol_alt_bn128_group_op (ALT_BN128_PAIRING_LE , input as * const _ as * const u8 , input . len () as u64 , result_buffer . as_mut_ptr () ,) ; match result { 0 => { result_buffer . set_len (ALT_BN128_PAIRING_OUTPUT_SIZE) ; Ok (result_buffer) } _ => Err (AltBn128Error :: UnexpectedError) , } } } }
};
}

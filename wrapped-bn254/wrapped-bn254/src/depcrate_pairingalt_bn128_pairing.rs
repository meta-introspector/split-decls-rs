// Generated macro for alt_bn128_pairing (function)
macro_rules! Depcrate_pairingalt_bn128_pairing {
() => {
// Module: crate::pairing
// Provides: {"alt_bn128_pairing"}
// Dependencies: {}
# [deprecated (since = "3.1.0" , note = "Please use `alt_bn128_pairing_be` instead")] # [allow (deprecated)] # [inline (always)] pub fn alt_bn128_pairing (input : & [u8]) -> Result < Vec < u8 > , AltBn128Error > { # [cfg (not (target_os = "solana"))] { alt_bn128_versioned_pairing (VersionedPairing :: V0 , input , Endianness :: BE) } # [cfg (target_os = "solana")] { if input . len () . checked_rem (ALT_BN128_PAIRING_ELEMENT_LEN) . is_none () { return Err (AltBn128Error :: InvalidInputData) ; } let mut result_buffer = [0u8 ; 32] ; let result = unsafe { syscalls :: sol_alt_bn128_group_op (ALT_BN128_PAIRING , input as * const _ as * const u8 , input . len () as u64 , & mut result_buffer as * mut _ as * mut u8 ,) } ; match result { 0 => Ok (result_buffer . to_vec ()) , _ => Err (AltBn128Error :: UnexpectedError) , } } }
};
}

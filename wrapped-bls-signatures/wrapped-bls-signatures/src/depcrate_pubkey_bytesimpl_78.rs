// Generated macro for impl_78 (impl)
macro_rules! Depcrate_pubkey_bytesimpl_78 {
() => {
// Module: crate::pubkey::bytes
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl AsPubkey for [u8 ; BLS_PUBLIC_KEY_AFFINE_SIZE] { fn try_as_affine (& self) -> Result < Pubkey , BlsError > { Ok (Pubkey (* self)) } }
};
}

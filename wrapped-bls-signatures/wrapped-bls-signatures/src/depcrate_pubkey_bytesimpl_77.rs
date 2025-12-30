// Generated macro for impl_77 (impl)
macro_rules! Depcrate_pubkey_bytesimpl_77 {
() => {
// Module: crate::pubkey::bytes
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl AsPubkey for [u8 ; BLS_PUBLIC_KEY_COMPRESSED_SIZE] { fn try_as_affine (& self) -> Result < Pubkey , BlsError > { let compressed = PubkeyCompressed (* self) ; Pubkey :: try_from (compressed) } }
};
}

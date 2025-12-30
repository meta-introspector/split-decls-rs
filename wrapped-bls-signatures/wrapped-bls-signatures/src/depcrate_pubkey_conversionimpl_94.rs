// Generated macro for impl_94 (impl)
macro_rules! Depcrate_pubkey_conversionimpl_94 {
() => {
// Module: crate::pubkey::conversion
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl From < & PubkeyProjective > for [u8 ; BLS_PUBLIC_KEY_AFFINE_SIZE] { fn from (pubkey : & PubkeyProjective) -> Self { let pubkey_affine : Pubkey = (* pubkey) . into () ; pubkey_affine . 0 } }
};
}

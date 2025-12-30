// Generated macro for AsPubkey (trait)
macro_rules! Depcrate_pubkey_bytesAsPubkey {
() => {
// Module: crate::pubkey::bytes
// Provides: {"AsPubkey"}
// Dependencies: {}
# [doc = " A trait for types that can be converted into a `Pubkey` (affine/uncompressed bytes)."] # [cfg (not (target_os = "solana"))] pub trait AsPubkey { # [doc = " Attempt to convert the type into a `Pubkey`."] fn try_as_affine (& self) -> Result < Pubkey , BlsError > ; }
};
}

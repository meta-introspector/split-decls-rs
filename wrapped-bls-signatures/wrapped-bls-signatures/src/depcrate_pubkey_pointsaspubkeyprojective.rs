// Generated macro for AsPubkeyProjective (trait)
macro_rules! Depcrate_pubkey_pointsAsPubkeyProjective {
() => {
// Module: crate::pubkey::points
// Provides: {"AsPubkeyProjective"}
// Dependencies: {}
# [doc = " A trait for types that can be converted into a `PubkeyProjective`."] # [cfg (not (target_os = "solana"))] pub trait AsPubkeyProjective { # [doc = " Attempt to convert the type into a `PubkeyProjective`."] fn try_as_projective (& self) -> Result < PubkeyProjective , BlsError > ; }
};
}

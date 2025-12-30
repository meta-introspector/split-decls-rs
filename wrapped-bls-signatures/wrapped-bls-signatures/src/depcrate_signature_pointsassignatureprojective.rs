// Generated macro for AsSignatureProjective (trait)
macro_rules! Depcrate_signature_pointsAsSignatureProjective {
() => {
// Module: crate::signature::points
// Provides: {"AsSignatureProjective"}
// Dependencies: {}
# [doc = " A trait for types that can be converted into a `SignatureProjective`."] # [cfg (not (target_os = "solana"))] pub trait AsSignatureProjective { # [doc = " Attempt to convert the type into a `SignatureProjective`."] fn try_as_projective (& self) -> Result < SignatureProjective , BlsError > ; }
};
}

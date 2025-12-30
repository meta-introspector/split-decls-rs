// Generated macro for AsSignature (trait)
macro_rules! Depcrate_signature_bytesAsSignature {
() => {
// Module: crate::signature::bytes
// Provides: {"AsSignature"}
// Dependencies: {}
# [doc = " A trait for types that can be converted into a `Signature` (affine/uncompressed bytes)."] # [cfg (not (target_os = "solana"))] pub trait AsSignature { # [doc = " Attempt to convert the type into a `Signature`."] fn try_as_affine (& self) -> Result < Signature , BlsError > ; }
};
}

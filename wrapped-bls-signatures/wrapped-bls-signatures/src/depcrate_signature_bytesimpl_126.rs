// Generated macro for impl_126 (impl)
macro_rules! Depcrate_signature_bytesimpl_126 {
() => {
// Module: crate::signature::bytes
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl AsSignature for [u8 ; BLS_SIGNATURE_AFFINE_SIZE] { fn try_as_affine (& self) -> Result < Signature , BlsError > { Ok (Signature (* self)) } }
};
}

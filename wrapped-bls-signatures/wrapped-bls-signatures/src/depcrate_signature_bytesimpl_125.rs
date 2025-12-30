// Generated macro for impl_125 (impl)
macro_rules! Depcrate_signature_bytesimpl_125 {
() => {
// Module: crate::signature::bytes
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl AsSignature for [u8 ; BLS_SIGNATURE_COMPRESSED_SIZE] { fn try_as_affine (& self) -> Result < Signature , BlsError > { let compressed = SignatureCompressed (* self) ; Signature :: try_from (compressed) } }
};
}

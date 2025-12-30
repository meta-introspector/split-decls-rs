// Generated macro for impl_285 (impl)
macro_rules! Depcrate_documentimpl_285 {
() => {
// Module: crate::document
// Provides: {"impl_285"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl TryFrom < & [u8] > for SecretDocument { type Error = Error ; fn try_from (der_bytes : & [u8]) -> Result < Self , Error > { Document :: try_from (der_bytes) . map (Self) } }
};
}

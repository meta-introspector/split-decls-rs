// Generated macro for impl_286 (impl)
macro_rules! Depcrate_documentimpl_286 {
() => {
// Module: crate::document
// Provides: {"impl_286"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl TryFrom < Vec < u8 > > for SecretDocument { type Error = Error ; fn try_from (der_bytes : Vec < u8 >) -> Result < Self , Error > { Document :: try_from (der_bytes) . map (Self) } }
};
}

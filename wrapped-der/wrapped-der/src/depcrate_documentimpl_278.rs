// Generated macro for impl_278 (impl)
macro_rules! Depcrate_documentimpl_278 {
() => {
// Module: crate::document
// Provides: {"impl_278"}
// Dependencies: {}
impl TryFrom < & [u8] > for Document { type Error = Error ; fn try_from (der_bytes : & [u8]) -> Result < Self , Error > { Self :: from_der (der_bytes) } }
};
}

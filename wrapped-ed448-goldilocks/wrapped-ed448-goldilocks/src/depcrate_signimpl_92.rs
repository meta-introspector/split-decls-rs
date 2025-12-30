// Generated macro for impl_92 (impl)
macro_rules! Depcrate_signimpl_92 {
() => {
// Module: crate::sign
// Provides: {"impl_92"}
// Dependencies: {}
impl From < InnerSignature > for Signature { fn from (inner : InnerSignature) -> Self { let mut s = [0u8 ; SECRET_KEY_LENGTH] ; s . copy_from_slice (& inner . s . to_bytes_rfc_8032 ()) ; Self :: from_components (inner . r . to_bytes () , s) } }
};
}

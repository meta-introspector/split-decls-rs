// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl TryFrom < & [u8] > for Signature { type Error = signature :: Error ; fn try_from (bytes : & [u8]) -> signature :: Result < Self > { Self :: from_der (bytes) . map_err (signature :: Error :: from_source) } }
};
}

// Generated macro for impl_98 (impl)
macro_rules! Depcrateimpl_98 {
() => {
// Module: crate
// Provides: {"impl_98"}
// Dependencies: {}
impl TryFrom < & [u8] > for Signature { type Error = signature :: Error ; fn try_from (bytes : & [u8]) -> signature :: Result < Self > { Self :: from_der (bytes) . map_err (signature :: Error :: from_source) } }
};
}

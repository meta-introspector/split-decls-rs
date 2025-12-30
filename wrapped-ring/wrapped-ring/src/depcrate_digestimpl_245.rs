// Generated macro for impl_245 (impl)
macro_rules! Depcrate_digestimpl_245 {
() => {
// Module: crate::digest
// Provides: {"impl_245"}
// Dependencies: {}
impl AsRef < [u8] > for Digest { # [inline (always)] fn as_ref (& self) -> & [u8] { & self . value . 0 [.. self . algorithm . output_len ()] } }
};
}

// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl From < & str > for ByteString { # [inline] fn from (value : & str) -> Self { Self (Bytes :: copy_from_slice (value . as_ref ())) } }
};
}

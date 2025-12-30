// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl From < Box < str > > for ByteString { # [inline] fn from (value : Box < str >) -> Self { Self (Bytes :: from (value . into_boxed_bytes ())) } }
};
}

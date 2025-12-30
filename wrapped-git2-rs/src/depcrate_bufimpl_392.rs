// Generated macro for impl_392 (impl)
macro_rules! Depcrate_bufimpl_392 {
() => {
// Module: crate::buf
// Provides: {"impl_392"}
// Dependencies: {}
impl Binding for Buf { type Raw = * mut raw :: git_buf ; unsafe fn from_raw (raw : * mut raw :: git_buf) -> Buf { Buf { raw : * raw } } fn raw (& self) -> * mut raw :: git_buf { & self . raw as * const _ as * mut _ } }
};
}

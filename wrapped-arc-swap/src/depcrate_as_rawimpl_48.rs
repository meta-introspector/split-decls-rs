// Generated macro for impl_48 (impl)
macro_rules! Depcrate_as_rawimpl_48 {
() => {
// Module: crate::as_raw
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : RefCnt > AsRaw < T :: Base > for Guard < T > { fn as_raw (& self) -> * mut T :: Base { T :: as_ptr (self) } }
};
}

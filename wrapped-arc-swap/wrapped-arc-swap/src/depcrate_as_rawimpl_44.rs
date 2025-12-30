// Generated macro for impl_44 (impl)
macro_rules! Depcrate_as_rawimpl_44 {
() => {
// Module: crate::as_raw
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , T : RefCnt > AsRaw < T :: Base > for & 'a T { fn as_raw (& self) -> * mut T :: Base { T :: as_ptr (self) } }
};
}

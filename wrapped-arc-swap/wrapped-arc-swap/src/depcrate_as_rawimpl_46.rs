// Generated macro for impl_46 (impl)
macro_rules! Depcrate_as_rawimpl_46 {
() => {
// Module: crate::as_raw
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a , T : RefCnt > AsRaw < T :: Base > for & 'a Guard < T > { fn as_raw (& self) -> * mut T :: Base { T :: as_ptr (self) } }
};
}

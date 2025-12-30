// Generated macro for impl_298 (impl)
macro_rules! Depcrate_extensions_unicode_valueimpl_298 {
() => {
// Module: crate::extensions::unicode::value
// Provides: {"impl_298"}
// Dependencies: {}
impl PartialEq < & str > for Value { fn eq (& self , other : & & str) -> bool { writeable :: cmp_utf8 (self , other . as_bytes ()) . is_eq () } }
};
}

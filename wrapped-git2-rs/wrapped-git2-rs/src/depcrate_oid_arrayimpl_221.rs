// Generated macro for impl_221 (impl)
macro_rules! Depcrate_oid_arrayimpl_221 {
() => {
// Module: crate::oid_array
// Provides: {"impl_221"}
// Dependencies: {}
impl Binding for OidArray { type Raw = raw :: git_oidarray ; unsafe fn from_raw (raw : raw :: git_oidarray) -> OidArray { OidArray { raw } } fn raw (& self) -> raw :: git_oidarray { self . raw } }
};
}

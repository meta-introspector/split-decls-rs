// Generated macro for impl_798 (impl)
macro_rules! Depcrate_oidimpl_798 {
() => {
// Module: crate::oid
// Provides: {"impl_798"}
// Dependencies: {}
impl Binding for Oid { type Raw = * const raw :: git_oid ; unsafe fn from_raw (oid : * const raw :: git_oid) -> Oid { Oid { raw : * oid } } fn raw (& self) -> * const raw :: git_oid { & self . raw as * const _ } }
};
}

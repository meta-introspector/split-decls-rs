// Generated macro for impl_805 (impl)
macro_rules! Depcrate_oidimpl_805 {
() => {
// Module: crate::oid
// Provides: {"impl_805"}
// Dependencies: {}
impl Ord for Oid { fn cmp (& self , other : & Oid) -> Ordering { c_cmp_to_ordering (unsafe { raw :: git_oid_cmp (& self . raw , & other . raw) }) } }
};
}

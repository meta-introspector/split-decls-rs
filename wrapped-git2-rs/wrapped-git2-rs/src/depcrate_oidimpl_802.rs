// Generated macro for impl_802 (impl)
macro_rules! Depcrate_oidimpl_802 {
() => {
// Module: crate::oid
// Provides: {"impl_802"}
// Dependencies: {}
impl PartialEq for Oid { fn eq (& self , other : & Oid) -> bool { unsafe { raw :: git_oid_equal (& self . raw , & other . raw) != 0 } } }
};
}

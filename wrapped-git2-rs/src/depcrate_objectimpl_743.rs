// Generated macro for impl_743 (impl)
macro_rules! Depcrate_objectimpl_743 {
() => {
// Module: crate::object
// Provides: {"impl_743"}
// Dependencies: {}
impl < 'repo > Clone for Object < 'repo > { fn clone (& self) -> Object < 'repo > { let mut raw = ptr :: null_mut () ; unsafe { let rc = raw :: git_object_dup (& mut raw , self . raw) ; assert_eq ! (rc , 0) ; Binding :: from_raw (raw) } } }
};
}

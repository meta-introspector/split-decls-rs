// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_runtime_methodimpl_1113 {
() => {
// Module: crate::runtime::method
// Provides: {"impl_1113"}
// Dependencies: {}
impl MethodDescription { pub (crate) unsafe fn from_raw (raw : ffi :: objc_method_description) -> Option < Self > { let sel = raw . name ? ; if raw . types . is_null () { return None ; } let types = unsafe { CStr :: from_ptr (raw . types) } ; Some (Self { sel , types }) } }
};
}

// Generated macro for impl_1068 (impl)
macro_rules! Depcrate_runtime_mallocimpl_1068 {
() => {
// Module: crate::runtime::malloc
// Provides: {"impl_1068"}
// Dependencies: {}
impl MallocCStr { pub (crate) unsafe fn from_c_str (ptr : * mut c_char) -> Self { if ptr . is_null () { panic ! ("tried to construct MallocStr from a NULL pointer") ; } let cstr = unsafe { CStr :: from_ptr (ptr) } ; let ptr = NonNull :: from (cstr) ; Self { ptr } } }
};
}

// Generated macro for impl_1069 (impl)
macro_rules! Depcrate_runtime_mallocimpl_1069 {
() => {
// Module: crate::runtime::malloc
// Provides: {"impl_1069"}
// Dependencies: {}
impl Drop for MallocCStr { # [inline] fn drop (& mut self) { unsafe { ffi :: free (self . ptr . cast () . as_ptr ()) } ; } }
};
}

// Generated macro for impl_1063 (impl)
macro_rules! Depcrate_runtime_mallocimpl_1063 {
() => {
// Module: crate::runtime::malloc
// Provides: {"impl_1063"}
// Dependencies: {}
impl < T > Drop for MallocSlice < T > { # [allow (clippy :: len_zero)] fn drop (& mut self) { if self . len () != 0 { unsafe { ptr :: drop_in_place (self . ptr . as_ptr ()) } ; unsafe { ffi :: free (self . ptr . cast () . as_ptr ()) } ; } } }
};
}

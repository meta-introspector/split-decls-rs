// Generated macro for impl_1064 (impl)
macro_rules! Depcrate_runtime_mallocimpl_1064 {
() => {
// Module: crate::runtime::malloc
// Provides: {"impl_1064"}
// Dependencies: {}
impl < T > Deref for MallocSlice < T > { type Target = [T] ; # [inline] fn deref (& self) -> & [T] { unsafe { self . ptr . as_ref () } } }
};
}

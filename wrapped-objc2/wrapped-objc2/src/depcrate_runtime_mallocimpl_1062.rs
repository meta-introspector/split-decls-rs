// Generated macro for impl_1062 (impl)
macro_rules! Depcrate_runtime_mallocimpl_1062 {
() => {
// Module: crate::runtime::malloc
// Provides: {"impl_1062"}
// Dependencies: {}
impl < T > MallocSlice < T > { pub (crate) unsafe fn from_array (mut ptr : * mut T , len : usize) -> Self { if len == 0 { ptr = NonNull :: dangling () . as_ptr () ; } let ptr = ptr :: slice_from_raw_parts_mut (ptr , len) ; let ptr = NonNull :: new (ptr) . expect ("tried to construct MallocSlice from a NULL pointer") ; Self { ptr , _p : PhantomData , } } fn len (& self) -> usize { (* * self) . len () } }
};
}

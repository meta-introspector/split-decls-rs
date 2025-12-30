// Generated macro for impl_155 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_155 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_155"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , CFrom : Dim , CTo : Dim > Reallocator < T , Dyn , CFrom , Dyn , CTo > for DefaultAllocator { # [inline] unsafe fn reallocate_copy (rto : Dyn , cto : CTo , buf : VecStorage < T , Dyn , CFrom > ,) -> VecStorage < MaybeUninit < T > , Dyn , CTo > { let new_buf = buf . resize (rto . value () * cto . value ()) ; VecStorage :: new (rto , cto , new_buf) } }
};
}

// Generated macro for impl_157 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_157 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_157"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , RFrom : DimName , CTo : Dim > Reallocator < T , RFrom , Dyn , Dyn , CTo > for DefaultAllocator { # [inline] unsafe fn reallocate_copy (rto : Dyn , cto : CTo , buf : VecStorage < T , RFrom , Dyn > ,) -> VecStorage < MaybeUninit < T > , Dyn , CTo > { let new_buf = buf . resize (rto . value () * cto . value ()) ; VecStorage :: new (rto , cto , new_buf) } }
};
}

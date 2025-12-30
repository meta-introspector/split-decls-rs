// Generated macro for impl_158 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_158 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_158"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , RFrom : DimName , RTo : DimName > Reallocator < T , RFrom , Dyn , RTo , Dyn > for DefaultAllocator { # [inline] unsafe fn reallocate_copy (rto : RTo , cto : Dyn , buf : VecStorage < T , RFrom , Dyn > ,) -> VecStorage < MaybeUninit < T > , RTo , Dyn > { let new_buf = buf . resize (rto . value () * cto . value ()) ; VecStorage :: new (rto , cto , new_buf) } }
};
}

// Generated macro for impl_156 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_156 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_156"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , CFrom : Dim , RTo : DimName > Reallocator < T , Dyn , CFrom , RTo , Dyn > for DefaultAllocator { # [inline] unsafe fn reallocate_copy (rto : RTo , cto : Dyn , buf : VecStorage < T , Dyn , CFrom > ,) -> VecStorage < MaybeUninit < T > , RTo , Dyn > { let new_buf = buf . resize (rto . value () * cto . value ()) ; VecStorage :: new (rto , cto , new_buf) } }
};
}

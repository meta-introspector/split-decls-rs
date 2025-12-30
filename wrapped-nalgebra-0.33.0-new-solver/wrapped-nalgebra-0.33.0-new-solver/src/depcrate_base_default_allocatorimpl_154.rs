// Generated macro for impl_154 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_154 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_154"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , RTo , const RFROM : usize , const CFROM : usize > Reallocator < T , Const < RFROM > , Const < CFROM > , RTo , Dyn > for DefaultAllocator where RTo : DimName , { # [inline] unsafe fn reallocate_copy (rto : RTo , cto : Dyn , buf : ArrayStorage < T , RFROM , CFROM > ,) -> VecStorage < MaybeUninit < T > , RTo , Dyn > { let mut res = < Self as Allocator < RTo , Dyn > > :: allocate_uninit (rto , cto) ; let (rfrom , cfrom) = buf . shape () ; let len_from = rfrom . value () * cfrom . value () ; let len_to = rto . value () * cto . value () ; let len_copied = cmp :: min (len_from , len_to) ; ptr :: copy_nonoverlapping (buf . ptr () , res . ptr_mut () as * mut T , len_copied) ; buf . forget_elements () ; res } }
};
}

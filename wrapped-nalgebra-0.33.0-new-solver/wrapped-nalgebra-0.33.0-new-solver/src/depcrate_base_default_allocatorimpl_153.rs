// Generated macro for impl_153 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_153 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_153"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , CTo , const RFROM : usize , const CFROM : usize > Reallocator < T , Const < RFROM > , Const < CFROM > , Dyn , CTo > for DefaultAllocator where CTo : Dim , { # [inline] unsafe fn reallocate_copy (rto : Dyn , cto : CTo , buf : ArrayStorage < T , RFROM , CFROM > ,) -> VecStorage < MaybeUninit < T > , Dyn , CTo > { let mut res = < Self as Allocator < Dyn , CTo > > :: allocate_uninit (rto , cto) ; let (rfrom , cfrom) = buf . shape () ; let len_from = rfrom . value () * cfrom . value () ; let len_to = rto . value () * cto . value () ; let len_copied = cmp :: min (len_from , len_to) ; ptr :: copy_nonoverlapping (buf . ptr () , res . ptr_mut () as * mut T , len_copied) ; buf . forget_elements () ; res } }
};
}

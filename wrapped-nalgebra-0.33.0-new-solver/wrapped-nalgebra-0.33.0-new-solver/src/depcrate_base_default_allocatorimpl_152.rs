// Generated macro for impl_152 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_152 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_152"}
// Dependencies: {}
impl < T : Scalar , RFrom , CFrom , const RTO : usize , const CTO : usize > Reallocator < T , RFrom , CFrom , Const < RTO > , Const < CTO > > for DefaultAllocator where RFrom : Dim , CFrom : Dim , Self : Allocator < RFrom , CFrom > , { # [inline] unsafe fn reallocate_copy (rto : Const < RTO > , cto : Const < CTO > , buf : < Self as Allocator < RFrom , CFrom > > :: Buffer < T > ,) -> ArrayStorage < MaybeUninit < T > , RTO , CTO > { let mut res = < Self as Allocator < Const < RTO > , Const < CTO > > > :: allocate_uninit (rto , cto) ; let (rfrom , cfrom) = buf . shape () ; let len_from = rfrom . value () * cfrom . value () ; let len_to = rto . value () * cto . value () ; let len_copied = cmp :: min (len_from , len_to) ; ptr :: copy_nonoverlapping (buf . ptr () , res . ptr_mut () as * mut T , len_copied) ; buf . forget_elements () ; res } }
};
}

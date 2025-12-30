// Generated macro for impl_15 (impl)
macro_rules! Depcrate_cmsgimpl_15 {
() => {
// Module: crate::cmsg
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a , M : MsgHdr > Iter < 'a , M > { # [doc = " # Safety"] # [doc = ""] # [doc = " `hdr` must hold a pointer to memory outliving `'a` which can be soundly read for the"] # [doc = " lifetime of the constructed `Iter` and contains a buffer of native cmsgs, i.e. is aligned"] pub (crate) unsafe fn new (hdr : & 'a M) -> Self { Self { hdr , cmsg : hdr . cmsg_first_hdr () . as_ref () , } } }
};
}

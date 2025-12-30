// Generated macro for impl_16 (impl)
macro_rules! Depcrate_cmsgimpl_16 {
() => {
// Module: crate::cmsg
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a , M : MsgHdr > Iterator for Iter < 'a , M > { type Item = & 'a M :: ControlMessage ; fn next (& mut self) -> Option < Self :: Item > { let current = self . cmsg . take () ? ; self . cmsg = unsafe { self . hdr . cmsg_nxt_hdr (current) . as_ref () } ; # [cfg (apple_fast)] { if current . len () < mem :: size_of :: < M :: ControlMessage > () { return None ; } } Some (current) } }
};
}

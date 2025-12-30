// Generated macro for impl_11 (impl)
macro_rules! Depcrate_cmsgimpl_11 {
() => {
// Module: crate::cmsg
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a , M : MsgHdr > Encoder < 'a , M > { # [doc = " # Safety"] # [doc = " - `hdr` must contain a suitably aligned pointer to a big enough buffer to hold control messages"] # [doc = "   bytes. All bytes of this buffer can be safely written."] # [doc = " - The `Encoder` must be dropped before `hdr` is passed to a system call, and must not be leaked."] pub (crate) unsafe fn new (hdr : & 'a mut M) -> Self { Self { cmsg : hdr . cmsg_first_hdr () . as_mut () , hdr , len : 0 , } } # [doc = " Append a control message to the buffer."] # [doc = ""] # [doc = " # Panics"] # [doc = " - If insufficient buffer space remains."] # [doc = " - If `T` has stricter alignment requirements than `M::ControlMessage`"] pub (crate) fn push < T : Copy > (& mut self , level : c_int , ty : c_int , value : T) { assert ! (mem :: align_of ::< T > () <= mem :: align_of ::< M :: ControlMessage > ()) ; let space = M :: ControlMessage :: cmsg_space (mem :: size_of_val (& value)) ; assert ! (self . hdr . control_len () >= self . len + space , "control message buffer too small. Required: {}, Available: {}" , self . len + space , self . hdr . control_len ()) ; let cmsg = self . cmsg . take () . expect ("no control buffer space remaining") ; cmsg . set (level , ty , M :: ControlMessage :: cmsg_len (mem :: size_of_val (& value)) ,) ; unsafe { ptr :: write (cmsg . cmsg_data () as * const T as * mut T , value) ; } self . len += space ; self . cmsg = unsafe { self . hdr . cmsg_nxt_hdr (cmsg) . as_mut () } ; } # [doc = " Finishes appending control messages to the buffer"] pub (crate) fn finish (self) { } }
};
}

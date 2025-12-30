// Generated macro for decode (function)
macro_rules! Depcrate_cmsgdecode {
() => {
// Module: crate::cmsg
// Provides: {"decode"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " `cmsg` must refer to a native cmsg containing a payload of type `T`"] pub (crate) unsafe fn decode < T : Copy , C : CMsgHdr > (cmsg : & impl CMsgHdr) -> T { assert ! (mem :: align_of ::< T > () <= mem :: align_of ::< C > ()) ; debug_assert_eq ! (cmsg . len () , C :: cmsg_len (mem :: size_of ::< T > ())) ; ptr :: read (cmsg . cmsg_data () as * const T) }
};
}

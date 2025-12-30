// Generated macro for Encoder (struct)
macro_rules! Depcrate_cmsgEncoder {
() => {
// Module: crate::cmsg
// Provides: {"Encoder"}
// Dependencies: {}
# [doc = " Helper to encode a series of control messages (native \"cmsgs\") to a buffer for use in `sendmsg`"] # [doc = ""] # [doc = " The operation must be \"finished\" for the native msghdr to be usable, either by calling `finish`"] # [doc = " explicitly or by dropping the `Encoder`."] pub (crate) struct Encoder < 'a , M : MsgHdr > { hdr : & 'a mut M , cmsg : Option < & 'a mut M :: ControlMessage > , len : usize , }
};
}

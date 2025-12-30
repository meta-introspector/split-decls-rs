// Generated macro for MsgHdr (trait)
macro_rules! Depcrate_cmsgMsgHdr {
() => {
// Module: crate::cmsg
// Provides: {"MsgHdr"}
// Dependencies: {}
pub (crate) trait MsgHdr { type ControlMessage : CMsgHdr ; fn cmsg_first_hdr (& self) -> * mut Self :: ControlMessage ; fn cmsg_nxt_hdr (& self , cmsg : & Self :: ControlMessage) -> * mut Self :: ControlMessage ; # [doc = " Sets the number of control messages added to this `struct msghdr`."] # [doc = ""] # [doc = " Note that this is a destructive operation and should only be done as a finalisation"] # [doc = " step."] fn set_control_len (& mut self , len : usize) ; fn control_len (& self) -> usize ; }
};
}

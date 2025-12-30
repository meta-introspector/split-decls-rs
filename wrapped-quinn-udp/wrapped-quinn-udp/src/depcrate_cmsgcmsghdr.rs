// Generated macro for CMsgHdr (trait)
macro_rules! Depcrate_cmsgCMsgHdr {
() => {
// Module: crate::cmsg
// Provides: {"CMsgHdr"}
// Dependencies: {}
pub (crate) trait CMsgHdr { fn cmsg_len (length : usize) -> usize ; fn cmsg_space (length : usize) -> usize ; fn cmsg_data (& self) -> * mut c_uchar ; fn set (& mut self , level : c_int , ty : c_int , len : usize) ; fn len (& self) -> usize ; }
};
}

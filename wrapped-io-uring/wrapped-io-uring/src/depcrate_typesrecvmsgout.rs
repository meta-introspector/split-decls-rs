// Generated macro for RecvMsgOut (struct)
macro_rules! Depcrate_typesRecvMsgOut {
() => {
// Module: crate::types
// Provides: {"RecvMsgOut"}
// Dependencies: {}
# [doc = " Helper structure for parsing the result of a multishot [`opcode::RecvMsg`](crate::opcode::RecvMsg)."] # [derive (Debug)] pub struct RecvMsgOut < 'buf > { header : sys :: io_uring_recvmsg_out , # [doc = " The fixed length of the name field, in bytes."] # [doc = ""] # [doc = " If the incoming name data is larger than this, it gets truncated to this."] # [doc = " If it is smaller, it gets 0-padded to fill the whole field. In either case,"] # [doc = " this fixed amount of space is reserved in the result buffer."] msghdr_name_len : usize , name_data : & 'buf [u8] , control_data : & 'buf [u8] , payload_data : & 'buf [u8] , }
};
}

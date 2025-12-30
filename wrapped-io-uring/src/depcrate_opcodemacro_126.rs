// Generated macro for macro_126 (macro)
macro_rules! Depcrate_opcodemacro_126 {
() => {
// Module: crate::opcode
// Provides: {"macro_126"}
// Dependencies: {}
opcode ! { # [doc = " Receive a bundle of buffers from a socket."] # [doc = ""] # [doc = " Parameter"] # [doc = "     buf_group: The id of the provided buffer pool to use for the bundle."] # [doc = ""] # [doc = " Note that as of kernel 6.10 first recv always gets a single buffer, while second"] # [doc = " obtains the bundle of remaining buffers. This behavior may change in the future."] # [doc = ""] # [doc = " Bundle variant is available since kernel 6.10"] pub struct RecvBundle { fd : { impl sealed :: UseFixed } , buf_group : { u16 } , ;; flags : i32 = 0 } pub const CODE = sys :: IORING_OP_RECV ; pub fn build (self) -> Entry { let RecvBundle { fd , buf_group , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_3 . msg_flags = flags as _ ; sqe . __bindgen_anon_4 . buf_group = buf_group ; sqe . flags |= crate :: squeue :: Flags :: BUFFER_SELECT . bits () ; sqe . ioprio |= sys :: IORING_RECVSEND_BUNDLE as u16 ; Entry (sqe) } }
};
}

macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_72 {
    () => {
        deps!();
        opcode ! { # [doc = " Receive multiple messages from a socket, equivalent to `recv(2)`."] # [doc = ""] # [doc = " Parameter:"] # [doc = "     buf_group: The id of the provided buffer pool to use for each received message."] # [doc = ""] # [doc = " MSG_WAITALL should not be set in flags."] # [doc = ""] # [doc = " The multishot version allows the application to issue a single receive request, which"] # [doc = " repeatedly posts a CQE when data is available. Each CQE will take a buffer out of a"] # [doc = " provided buffer pool for receiving. The application should check the flags of each CQE,"] # [doc = " regardless of its result. If a posted CQE does not have the IORING_CQE_F_MORE flag set then"] # [doc = " the multishot receive will be done and the application should issue a new request."] # [doc = ""] # [doc = " Multishot variants are available since kernel 6.0."] pub struct RecvMulti { fd : { impl sealed :: UseFixed } , buf_group : { u16 } , ;; flags : i32 = 0 , } pub const CODE = sys :: IORING_OP_RECV ; pub fn build (self) -> Entry { let RecvMulti { fd , buf_group , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_3 . msg_flags = flags as _ ; sqe . __bindgen_anon_4 . buf_group = buf_group ; sqe . flags |= crate :: squeue :: Flags :: BUFFER_SELECT . bits () ; sqe . ioprio = sys :: IORING_RECV_MULTISHOT as _ ; Entry (sqe) } }
    };
}

macro_72!()
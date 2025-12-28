macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        opcode ! { # [doc = " Receive a message from a socket, equivalent to `recv(2)`."] pub struct Recv { fd : { impl sealed :: UseFixed } , buf : { * mut u8 } , len : { u32 } , ;; flags : i32 = 0 , buf_group : u16 = 0 } pub const CODE = sys :: IORING_OP_RECV ; pub fn build (self) -> Entry { let Recv { fd , buf , len , flags , buf_group } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = buf as _ ; sqe . len = len ; sqe . __bindgen_anon_3 . msg_flags = flags as _ ; sqe . __bindgen_anon_4 . buf_group = buf_group ; Entry (sqe) } }
    };
}

macro_71!();
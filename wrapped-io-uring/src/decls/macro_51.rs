macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_51 {
    () => {
        deps!();
        opcode ! { # [doc = " Receive a message on a socket, equivalent to `recvmsg(2)`."] # [doc = ""] # [doc = " See also the description of [`SendMsg`]."] # [derive (Debug)] pub struct RecvMsg { fd : { impl sealed :: UseFixed } , msg : { * mut libc :: msghdr } , ;; ioprio : u16 = 0 , flags : u32 = 0 , buf_group : u16 = 0 } pub const CODE = sys :: IORING_OP_RECVMSG ; pub fn build (self) -> Entry { let RecvMsg { fd , msg , ioprio , flags , buf_group } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . ioprio = ioprio ; sqe . __bindgen_anon_2 . addr = msg as _ ; sqe . len = 1 ; sqe . __bindgen_anon_3 . msg_flags = flags ; sqe . __bindgen_anon_4 . buf_group = buf_group ; Entry (sqe) } }
    };
}

macro_51!()
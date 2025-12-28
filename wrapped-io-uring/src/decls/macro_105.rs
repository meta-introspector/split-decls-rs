macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_105 {
    () => {
        deps!();
        opcode ! { # [doc = " Send a bundle of messages on a socket in a single request."] pub struct SendBundle { fd : { impl sealed :: UseFixed } , buf_group : { u16 } , ;; flags : i32 = 0 , len : u32 = 0 } pub const CODE = sys :: IORING_OP_SEND ; pub fn build (self) -> Entry { let SendBundle { fd , len , flags , buf_group } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . len = len ; sqe . __bindgen_anon_3 . msg_flags = flags as _ ; sqe . ioprio |= sys :: IORING_RECVSEND_BUNDLE as u16 ; sqe . flags |= crate :: squeue :: Flags :: BUFFER_SELECT . bits () ; sqe . __bindgen_anon_4 . buf_group = buf_group ; Entry (sqe) } }
    };
}

macro_105!()
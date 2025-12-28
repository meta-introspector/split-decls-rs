macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_110 {
    () => {
        deps!();
        opcode ! { # [doc = " Issue the zerocopy equivalent of a `recv(2)` system call."] pub struct RecvZc { fd : { impl sealed :: UseFixed } , len : { u32 } , ;; ifq : u32 = 0 , ioprio : u16 = 0 , } pub const CODE = sys :: IORING_OP_RECV_ZC ; pub fn build (self) -> Entry { let Self { fd , len , ifq , ioprio } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . len = len ; sqe . ioprio = ioprio | sys :: IORING_RECV_MULTISHOT as u16 ; sqe . __bindgen_anon_5 . zcrx_ifq_idx = ifq ; Entry (sqe) } }
    };
}

macro_110!()
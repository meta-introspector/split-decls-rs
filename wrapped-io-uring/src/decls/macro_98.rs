macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_98 {
    () => {
        deps!();
        opcode ! { # [doc = " Issue the equivalent of `pread(2)` with multi-shot semantics."] pub struct ReadMulti { fd : { impl sealed :: UseFixed } , len : { u32 } , buf_group : { u16 } , ;; offset : u64 = 0 , } pub const CODE = sys :: IORING_OP_READ_MULTISHOT ; pub fn build (self) -> Entry { let Self { fd , len , buf_group , offset } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . off = offset ; sqe . len = len ; sqe . __bindgen_anon_4 . buf_group = buf_group ; sqe . flags = crate :: squeue :: Flags :: BUFFER_SELECT . bits () ; Entry (sqe) } }
    };
}

macro_98!();
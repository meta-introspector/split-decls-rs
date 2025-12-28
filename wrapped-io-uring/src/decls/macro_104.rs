macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_104 {
    () => {
        deps!();
        opcode ! { # [doc = " Perform file truncation, equivalent to `ftruncate(2)`."] # [derive (Debug)] pub struct Ftruncate { fd : { impl sealed :: UseFixed } , len : { u64 } , ;; } pub const CODE = sys :: IORING_OP_FTRUNCATE ; pub fn build (self) -> Entry { let Ftruncate { fd , len } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . off = len ; Entry (sqe) } }
    };
}

macro_104!()
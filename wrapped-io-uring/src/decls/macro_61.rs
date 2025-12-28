macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_61 {
    () => {
        deps!();
        opcode ! { # [doc = " Preallocate or deallocate space to a file, equivalent to `fallocate(2)`."] pub struct Fallocate { fd : { impl sealed :: UseFixed } , len : { u64 } , ;; offset : u64 = 0 , mode : i32 = 0 } pub const CODE = sys :: IORING_OP_FALLOCATE ; pub fn build (self) -> Entry { let Fallocate { fd , len , offset , mode } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = len ; sqe . len = mode as _ ; sqe . __bindgen_anon_1 . off = offset ; Entry (sqe) } }
    };
}

macro_61!()
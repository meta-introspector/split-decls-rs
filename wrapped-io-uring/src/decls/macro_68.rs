macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_68 {
    () => {
        deps!();
        opcode ! { # [doc = " Predeclare an access pattern for file data, equivalent to `posix_fadvise(2)`."] pub struct Fadvise { fd : { impl sealed :: UseFixed } , len : { libc :: off_t } , advice : { i32 } , ;; offset : u64 = 0 , } pub const CODE = sys :: IORING_OP_FADVISE ; pub fn build (self) -> Entry { let Fadvise { fd , len , advice , offset } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . len = len as _ ; sqe . __bindgen_anon_1 . off = offset ; sqe . __bindgen_anon_3 . fadvise_advice = advice as _ ; Entry (sqe) } }
    };
}

macro_68!();
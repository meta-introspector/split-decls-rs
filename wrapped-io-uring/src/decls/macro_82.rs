macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_82 {
    () => {
        deps!();
        opcode ! { # [doc = " Make a directory, equivalent to `mkdirat(2)`."] pub struct MkDirAt { dirfd : { impl sealed :: UseFd } , pathname : { * const libc :: c_char } , ;; mode : libc :: mode_t = 0 } pub const CODE = sys :: IORING_OP_MKDIRAT ; pub fn build (self) -> Entry { let MkDirAt { dirfd , pathname , mode } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = dirfd ; sqe . __bindgen_anon_2 . addr = pathname as _ ; sqe . len = mode ; Entry (sqe) } }
    };
}

macro_82!()
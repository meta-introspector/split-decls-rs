macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_84 {
    () => {
        deps!();
        opcode ! { # [doc = " Create a hard link, equivalent to `linkat(2)`."] pub struct LinkAt { olddirfd : { impl sealed :: UseFd } , oldpath : { * const libc :: c_char } , newdirfd : { impl sealed :: UseFd } , newpath : { * const libc :: c_char } , ;; flags : i32 = 0 } pub const CODE = sys :: IORING_OP_LINKAT ; pub fn build (self) -> Entry { let LinkAt { olddirfd , oldpath , newdirfd , newpath , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = olddirfd as _ ; sqe . __bindgen_anon_2 . addr = oldpath as _ ; sqe . len = newdirfd as _ ; sqe . __bindgen_anon_1 . addr2 = newpath as _ ; sqe . __bindgen_anon_3 . hardlink_flags = flags as _ ; Entry (sqe) } }
    };
}

macro_84!()
macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_80 {
    () => {
        deps!();
        opcode ! { pub struct RenameAt { olddirfd : { impl sealed :: UseFd } , oldpath : { * const libc :: c_char } , newdirfd : { impl sealed :: UseFd } , newpath : { * const libc :: c_char } , ;; flags : u32 = 0 } pub const CODE = sys :: IORING_OP_RENAMEAT ; pub fn build (self) -> Entry { let RenameAt { olddirfd , oldpath , newdirfd , newpath , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = olddirfd ; sqe . __bindgen_anon_2 . addr = oldpath as _ ; sqe . len = newdirfd as _ ; sqe . __bindgen_anon_1 . off = newpath as _ ; sqe . __bindgen_anon_3 . rename_flags = flags ; Entry (sqe) } }
    };
}

macro_80!();
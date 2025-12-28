macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_69 {
    () => {
        deps!();
        opcode ! { # [doc = " Give advice about use of memory, equivalent to `madvise(2)`."] pub struct Madvise { addr : { * const libc :: c_void } , len : { libc :: off_t } , advice : { i32 } , ;; } pub const CODE = sys :: IORING_OP_MADVISE ; pub fn build (self) -> Entry { let Madvise { addr , len , advice } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = addr as _ ; sqe . len = len as _ ; sqe . __bindgen_anon_3 . fadvise_advice = advice as _ ; Entry (sqe) } }
    };
}

macro_69!();
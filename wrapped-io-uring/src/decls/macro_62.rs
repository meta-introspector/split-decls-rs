macro_rules! deps {
    () => {
        Entry!();
        DestinationSlot!();
    };
}

macro_rules! macro_62 {
    () => {
        deps!();
        opcode ! { # [doc = " Open a file, equivalent to `openat(2)`."] pub struct OpenAt { dirfd : { impl sealed :: UseFd } , pathname : { * const libc :: c_char } , ;; file_index : Option < types :: DestinationSlot > = None , flags : i32 = 0 , mode : libc :: mode_t = 0 } pub const CODE = sys :: IORING_OP_OPENAT ; pub fn build (self) -> Entry { let OpenAt { dirfd , pathname , file_index , flags , mode } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = dirfd ; sqe . __bindgen_anon_2 . addr = pathname as _ ; sqe . len = mode ; sqe . __bindgen_anon_3 . open_flags = flags as _ ; if let Some (dest) = file_index { sqe . __bindgen_anon_5 . file_index = dest . kernel_index_arg () ; } Entry (sqe) } }
    };
}

macro_62!();
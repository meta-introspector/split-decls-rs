macro_rules! deps {
    () => {
        OpenHow!();
        DestinationSlot!();
        Entry!();
    };
}

macro_rules! macro_73 {
    () => {
        deps!();
        opcode ! { # [doc = " Open a file, equivalent to `openat2(2)`."] pub struct OpenAt2 { dirfd : { impl sealed :: UseFd } , pathname : { * const libc :: c_char } , how : { * const types :: OpenHow } ;; file_index : Option < types :: DestinationSlot > = None , } pub const CODE = sys :: IORING_OP_OPENAT2 ; pub fn build (self) -> Entry { let OpenAt2 { dirfd , pathname , how , file_index } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = dirfd ; sqe . __bindgen_anon_2 . addr = pathname as _ ; sqe . len = mem :: size_of ::< sys :: open_how > () as _ ; sqe . __bindgen_anon_1 . off = how as _ ; if let Some (dest) = file_index { sqe . __bindgen_anon_5 . file_index = dest . kernel_index_arg () ; } Entry (sqe) } }
    };
}

macro_73!();
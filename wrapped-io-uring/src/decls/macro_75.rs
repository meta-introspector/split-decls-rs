macro_rules! deps {
    () => {
        Fixed!();
        Entry!();
        Fd!();
    };
}

macro_rules! macro_75 {
    () => {
        deps!();
        opcode ! { # [doc = " Splice data to/from a pipe, equivalent to `splice(2)`."] # [doc = ""] # [doc = " if `fd_in` refers to a pipe, `off_in` must be `-1`;"] # [doc = " The description of `off_in` also applied to `off_out`."] pub struct Splice { fd_in : { impl sealed :: UseFixed } , off_in : { i64 } , fd_out : { impl sealed :: UseFixed } , off_out : { i64 } , len : { u32 } , ;; # [doc = " see man `splice(2)` for description of flags."] flags : u32 = 0 } pub const CODE = sys :: IORING_OP_SPLICE ; pub fn build (self) -> Entry { let Splice { fd_in , off_in , fd_out , off_out , len , mut flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd_out) ; sqe . len = len ; sqe . __bindgen_anon_1 . off = off_out as _ ; sqe . __bindgen_anon_5 . splice_fd_in = match fd_in { sealed :: Target :: Fd (fd) => fd , sealed :: Target :: Fixed (idx) => { flags |= sys :: SPLICE_F_FD_IN_FIXED ; idx as _ } } ; sqe . __bindgen_anon_2 . splice_off_in = off_in as _ ; sqe . __bindgen_anon_3 . splice_flags = flags ; Entry (sqe) } }
    };
}

macro_75!()
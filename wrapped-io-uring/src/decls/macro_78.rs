macro_rules! deps {
    () => {
        Fd!();
        Fixed!();
        Entry!();
    };
}

macro_rules! macro_78 {
    () => {
        deps!();
        opcode ! { # [doc = " Duplicate pipe content, equivalent to `tee(2)`."] pub struct Tee { fd_in : { impl sealed :: UseFixed } , fd_out : { impl sealed :: UseFixed } , len : { u32 } ;; flags : u32 = 0 } pub const CODE = sys :: IORING_OP_TEE ; pub fn build (self) -> Entry { let Tee { fd_in , fd_out , len , mut flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd_out) ; sqe . len = len ; sqe . __bindgen_anon_5 . splice_fd_in = match fd_in { sealed :: Target :: Fd (fd) => fd , sealed :: Target :: Fixed (idx) => { flags |= sys :: SPLICE_F_FD_IN_FIXED ; idx as _ } } ; sqe . __bindgen_anon_3 . splice_flags = flags ; Entry (sqe) } }
    };
}

macro_78!()
macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_79 {
    () => {
        deps!();
        opcode ! { # [doc = " Shut down all or part of a full duplex connection on a socket, equivalent to `shutdown(2)`."] # [doc = " Available since kernel 5.11."] pub struct Shutdown { fd : { impl sealed :: UseFixed } , how : { i32 } , ;; } pub const CODE = sys :: IORING_OP_SHUTDOWN ; pub fn build (self) -> Entry { let Shutdown { fd , how } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . len = how as _ ; Entry (sqe) } }
    };
}

macro_79!()
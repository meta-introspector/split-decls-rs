macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_109 {
    () => {
        deps!();
        opcode ! { # [doc = " Listen on a socket, equivalent to `listen(2)`."] pub struct Listen { fd : { impl sealed :: UseFixed } , backlog : { i32 } , ;; } pub const CODE = sys :: IORING_OP_LISTEN ; pub fn build (self) -> Entry { let Listen { fd , backlog } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . len = backlog as _ ; Entry (sqe) } }
    };
}

macro_109!();
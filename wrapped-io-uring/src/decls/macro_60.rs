macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_60 {
    () => {
        deps!();
        opcode ! { # [doc = " Connect a socket, equivalent to `connect(2)`."] pub struct Connect { fd : { impl sealed :: UseFixed } , addr : { * const libc :: sockaddr } , addrlen : { libc :: socklen_t } ;; } pub const CODE = sys :: IORING_OP_CONNECT ; pub fn build (self) -> Entry { let Connect { fd , addr , addrlen } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = addr as _ ; sqe . __bindgen_anon_1 . off = addrlen as _ ; Entry (sqe) } }
    };
}

macro_60!()
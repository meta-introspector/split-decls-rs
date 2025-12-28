macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_111 {
    () => {
        deps!();
        opcode ! { # [doc = " Issue the equivalent of a `epoll_wait(2)` system call."] pub struct EpollWait { fd : { impl sealed :: UseFixed } , events : { * mut types :: epoll_event } , max_events : { u32 } , ;; flags : u32 = 0 , } pub const CODE = sys :: IORING_OP_EPOLL_WAIT ; pub fn build (self) -> Entry { let Self { fd , events , max_events , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = events as u64 ; sqe . len = max_events ; sqe . __bindgen_anon_3 . poll32_events = flags ; Entry (sqe) } }
    };
}

macro_111!()
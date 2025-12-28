macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        opcode ! { # [doc = " Modify an epoll file descriptor, equivalent to `epoll_ctl(2)`."] pub struct EpollCtl { epfd : { impl sealed :: UseFixed } , fd : { impl sealed :: UseFd } , op : { i32 } , ev : { * const types :: epoll_event } , ;; } pub const CODE = sys :: IORING_OP_EPOLL_CTL ; pub fn build (self) -> Entry { let EpollCtl { epfd , fd , op , ev } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = epfd) ; sqe . __bindgen_anon_2 . addr = ev as _ ; sqe . len = op as _ ; sqe . __bindgen_anon_1 . off = fd as _ ; Entry (sqe) } }
    };
}

macro_74!();
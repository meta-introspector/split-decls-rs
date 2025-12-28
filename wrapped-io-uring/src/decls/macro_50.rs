macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_50 {
    () => {
        deps!();
        opcode ! { # [doc = " Send a message on a socket, equivalent to `send(2)`."] # [doc = ""] # [doc = " fd must be set to the socket file descriptor, addr must contains a pointer to the msghdr"] # [doc = " structure, and flags holds the flags associated with the system call."] # [derive (Debug)] pub struct SendMsg { fd : { impl sealed :: UseFixed } , msg : { * const libc :: msghdr } , ;; ioprio : u16 = 0 , flags : u32 = 0 } pub const CODE = sys :: IORING_OP_SENDMSG ; pub fn build (self) -> Entry { let SendMsg { fd , msg , ioprio , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . ioprio = ioprio ; sqe . __bindgen_anon_2 . addr = msg as _ ; sqe . len = 1 ; sqe . __bindgen_anon_3 . msg_flags = flags ; Entry (sqe) } }
    };
}

macro_50!()
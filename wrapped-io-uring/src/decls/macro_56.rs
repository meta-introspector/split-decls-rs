macro_rules! deps {
    () => {
        DestinationSlot!();
        Entry!();
    };
}

macro_rules! macro_56 {
    () => {
        deps!();
        opcode ! { # [doc = " Accept a new connection on a socket, equivalent to `accept4(2)`."] pub struct Accept { fd : { impl sealed :: UseFixed } , addr : { * mut libc :: sockaddr } , addrlen : { * mut libc :: socklen_t } , ;; file_index : Option < types :: DestinationSlot > = None , flags : i32 = 0 } pub const CODE = sys :: IORING_OP_ACCEPT ; pub fn build (self) -> Entry { let Accept { fd , addr , addrlen , file_index , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = addr as _ ; sqe . __bindgen_anon_1 . addr2 = addrlen as _ ; sqe . __bindgen_anon_3 . accept_flags = flags as _ ; if let Some (dest) = file_index { sqe . __bindgen_anon_5 . file_index = dest . kernel_index_arg () ; } Entry (sqe) } }
    };
}

macro_56!();
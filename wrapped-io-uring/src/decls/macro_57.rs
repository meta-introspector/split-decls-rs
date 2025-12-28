macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_57 {
    () => {
        deps!();
        opcode ! { # [doc = " Set a socket option."] pub struct SetSockOpt { fd : { impl sealed :: UseFixed } , level : { u32 } , optname : { u32 } , optval : { * const libc :: c_void } , optlen : { u32 } , ;; flags : u32 = 0 } pub const CODE = sys :: IORING_OP_URING_CMD ; pub fn build (self) -> Entry { let SetSockOpt { fd , level , optname , optval , optlen , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . __bindgen_anon_1 . cmd_op = sys :: SOCKET_URING_OP_SETSOCKOPT ; sqe . __bindgen_anon_2 . __bindgen_anon_1 . level = level ; sqe . __bindgen_anon_2 . __bindgen_anon_1 . optname = optname ; sqe . __bindgen_anon_3 . uring_cmd_flags = flags ; sqe . __bindgen_anon_5 . optlen = optlen ; unsafe { * sqe . __bindgen_anon_6 . optval . as_mut () = optval as u64 } ; Entry (sqe) } }
    };
}

macro_57!()
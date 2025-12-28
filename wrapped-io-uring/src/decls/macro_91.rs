macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_91 {
    () => {
        deps!();
        opcode ! { # [doc = " A file/device-specific 16-byte command, akin (but not equivalent) to `ioctl(2)`."] pub struct UringCmd16 { fd : { impl sealed :: UseFixed } , cmd_op : { u32 } , ;; # [doc = " The `buf_index` is an index into an array of fixed buffers,"] # [doc = " and is only valid if fixed buffers were registered."] buf_index : Option < u16 > = None , # [doc = " Arbitrary command data."] cmd : [u8 ; 16] = [0u8 ; 16] } pub const CODE = sys :: IORING_OP_URING_CMD ; pub fn build (self) -> Entry { let UringCmd16 { fd , cmd_op , cmd , buf_index } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . __bindgen_anon_1 . cmd_op = cmd_op ; unsafe { * sqe . __bindgen_anon_6 . cmd . as_mut () . as_mut_ptr () . cast ::< [u8 ; 16] > () = cmd } ; if let Some (buf_index) = buf_index { sqe . __bindgen_anon_4 . buf_index = buf_index ; unsafe { sqe . __bindgen_anon_3 . uring_cmd_flags |= sys :: IORING_URING_CMD_FIXED ; } } Entry (sqe) } }
    };
}

macro_91!()
macro_rules! deps {
    () => {
        Entry!();
        Entry128!();
    };
}

macro_rules! macro_92 {
    () => {
        deps!();
        opcode ! { # [doc = " A file/device-specific 80-byte command, akin (but not equivalent) to `ioctl(2)`."] pub struct UringCmd80 { fd : { impl sealed :: UseFixed } , cmd_op : { u32 } , ;; # [doc = " The `buf_index` is an index into an array of fixed buffers,"] # [doc = " and is only valid if fixed buffers were registered."] buf_index : Option < u16 > = None , # [doc = " Arbitrary command data."] cmd : [u8 ; 80] = [0u8 ; 80] } pub const CODE = sys :: IORING_OP_URING_CMD ; pub fn build (self) -> Entry128 { let UringCmd80 { fd , cmd_op , cmd , buf_index } = self ; let cmd1 = cmd [.. 16] . try_into () . unwrap () ; let cmd2 = cmd [16 ..] . try_into () . unwrap () ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . __bindgen_anon_1 . cmd_op = cmd_op ; unsafe { * sqe . __bindgen_anon_6 . cmd . as_mut () . as_mut_ptr () . cast ::< [u8 ; 16] > () = cmd1 } ; if let Some (buf_index) = buf_index { sqe . __bindgen_anon_4 . buf_index = buf_index ; unsafe { sqe . __bindgen_anon_3 . uring_cmd_flags |= sys :: IORING_URING_CMD_FIXED ; } } Entry128 (Entry (sqe) , cmd2) } }
    };
}

macro_92!()
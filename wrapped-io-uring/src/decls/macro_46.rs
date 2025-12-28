macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_46 {
    () => {
        deps!();
        opcode ! { # [doc = " Write to a file from a fixed buffer that have been previously registered with"] # [doc = " [`Submitter::register_buffers`](crate::Submitter::register_buffers)."] # [doc = ""] # [doc = " The return values match those documented in the `pwritev2(2)` man pages."] # [derive (Debug)] pub struct WriteFixed { fd : { impl sealed :: UseFixed } , buf : { * const u8 } , len : { u32 } , buf_index : { u16 } , ;; ioprio : u16 = 0 , # [doc = " The offset of the file to write to."] offset : u64 = 0 , # [doc = " Specified for write operations, contains a bitwise OR of per-I/O flags, as described in"] # [doc = " the `pwritev2(2)` man page."] rw_flags : i32 = 0 } pub const CODE = sys :: IORING_OP_WRITE_FIXED ; pub fn build (self) -> Entry { let WriteFixed { fd , buf , len , offset , buf_index , ioprio , rw_flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . ioprio = ioprio ; sqe . __bindgen_anon_2 . addr = buf as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = offset ; sqe . __bindgen_anon_3 . rw_flags = rw_flags as _ ; sqe . __bindgen_anon_4 . buf_index = buf_index ; Entry (sqe) } }
    };
}

macro_46!();
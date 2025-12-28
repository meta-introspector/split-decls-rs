macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_42 {
    () => {
        deps!();
        opcode ! { # [doc = " Vectored read, equivalent to `preadv2(2)`."] # [derive (Debug)] pub struct Readv { fd : { impl sealed :: UseFixed } , iovec : { * const libc :: iovec } , len : { u32 } , ;; ioprio : u16 = 0 , offset : u64 = 0 , # [doc = " specified for read operations, contains a bitwise OR of per-I/O flags,"] # [doc = " as described in the `preadv2(2)` man page."] rw_flags : i32 = 0 , buf_group : u16 = 0 } pub const CODE = sys :: IORING_OP_READV ; pub fn build (self) -> Entry { let Readv { fd , iovec , len , offset , ioprio , rw_flags , buf_group } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . ioprio = ioprio ; sqe . __bindgen_anon_2 . addr = iovec as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = offset ; sqe . __bindgen_anon_3 . rw_flags = rw_flags as _ ; sqe . __bindgen_anon_4 . buf_group = buf_group ; Entry (sqe) } }
    };
}

macro_42!();
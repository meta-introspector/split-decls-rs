// Generated macro for macro_86 (macro)
macro_rules! Depcrate_opcodemacro_86 {
() => {
// Module: crate::opcode
// Provides: {"macro_86"}
// Dependencies: {}
opcode ! { # [doc = " Issue the equivalent of a `pread(2)` or `pwrite(2)` system call"] # [doc = ""] # [doc = " * `fd` is the file descriptor to be operated on,"] # [doc = " * `addr` contains the buffer in question,"] # [doc = " * `len` contains the length of the IO operation,"] # [doc = ""] # [doc = " These are non-vectored versions of the `IORING_OP_READV` and `IORING_OP_WRITEV` opcodes."] # [doc = " See also `read(2)` and `write(2)` for the general description of the related system call."] # [doc = ""] # [doc = " Available since 5.6."] pub struct Read { fd : { impl sealed :: UseFixed } , buf : { * mut u8 } , len : { u32 } , ;; # [doc = " `offset` contains the read or write offset."] # [doc = ""] # [doc = " If `fd` does not refer to a seekable file, `offset` must be set to zero."] # [doc = " If `offset` is set to `-1`, the offset will use (and advance) the file position,"] # [doc = " like the `read(2)` and `write(2)` system calls."] offset : u64 = 0 , ioprio : u16 = 0 , rw_flags : i32 = 0 , buf_group : u16 = 0 } pub const CODE = sys :: IORING_OP_READ ; pub fn build (self) -> Entry { let Read { fd , buf , len , offset , ioprio , rw_flags , buf_group } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . ioprio = ioprio ; sqe . __bindgen_anon_2 . addr = buf as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = offset ; sqe . __bindgen_anon_3 . rw_flags = rw_flags as _ ; sqe . __bindgen_anon_4 . buf_group = buf_group ; Entry (sqe) } }
};
}

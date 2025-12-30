// Generated macro for macro_65 (macro)
macro_rules! Depcrate_opcodemacro_65 {
() => {
// Module: crate::opcode
// Provides: {"macro_65"}
// Dependencies: {}
opcode ! { # [doc = " Read from a file into a fixed buffer that has been previously registered with"] # [doc = " [`Submitter::register_buffers`](crate::Submitter::register_buffers)."] # [doc = ""] # [doc = " The return values match those documented in the `preadv2(2)` man pages."] # [derive (Debug)] pub struct ReadFixed { fd : { impl sealed :: UseFixed } , buf : { * mut u8 } , len : { u32 } , buf_index : { u16 } , ;; ioprio : u16 = 0 , # [doc = " The offset of the file to read from."] offset : u64 = 0 , # [doc = " Specified for read operations, contains a bitwise OR of per-I/O flags, as described in"] # [doc = " the `preadv2(2)` man page."] rw_flags : i32 = 0 } pub const CODE = sys :: IORING_OP_READ_FIXED ; pub fn build (self) -> Entry { let ReadFixed { fd , buf , len , offset , buf_index , ioprio , rw_flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . ioprio = ioprio ; sqe . __bindgen_anon_2 . addr = buf as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = offset ; sqe . __bindgen_anon_3 . rw_flags = rw_flags as _ ; sqe . __bindgen_anon_4 . buf_index = buf_index ; Entry (sqe) } }
};
}

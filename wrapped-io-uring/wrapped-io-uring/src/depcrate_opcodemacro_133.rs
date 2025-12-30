// Generated macro for macro_133 (macro)
macro_rules! Depcrate_opcodemacro_133 {
() => {
// Module: crate::opcode
// Provides: {"macro_133"}
// Dependencies: {}
opcode ! { # [doc = " Vectored write from a fixed buffer, equivalent to `pwritev2(2)`."] pub struct WritevFixed { fd : { impl sealed :: UseFixed } , iovec : { * const :: libc :: iovec } , len : { u32 } , buf_index : { u16 } , ;; ioprio : u16 = 0 , offset : u64 = 0 , rw_flags : i32 = 0 , } pub const CODE = sys :: IORING_OP_WRITEV_FIXED ; pub fn build (self) -> Entry { let Self { fd , iovec , len , buf_index , offset , ioprio , rw_flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . off = offset as _ ; sqe . __bindgen_anon_2 . addr = iovec as _ ; sqe . len = len ; sqe . __bindgen_anon_4 . buf_index = buf_index ; sqe . ioprio = ioprio ; sqe . __bindgen_anon_3 . rw_flags = rw_flags as _ ; Entry (sqe) } }
};
}

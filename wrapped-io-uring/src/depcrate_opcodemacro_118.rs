// Generated macro for macro_118 (macro)
macro_rules! Depcrate_opcodemacro_118 {
() => {
// Module: crate::opcode
// Provides: {"macro_118"}
// Dependencies: {}
opcode ! { # [doc = " Issue the equivalent of `pread(2)` with multi-shot semantics."] pub struct ReadMulti { fd : { impl sealed :: UseFixed } , len : { u32 } , buf_group : { u16 } , ;; offset : u64 = 0 , } pub const CODE = sys :: IORING_OP_READ_MULTISHOT ; pub fn build (self) -> Entry { let Self { fd , len , buf_group , offset } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . off = offset ; sqe . len = len ; sqe . __bindgen_anon_4 . buf_group = buf_group ; sqe . flags = crate :: squeue :: Flags :: BUFFER_SELECT . bits () ; Entry (sqe) } }
};
}

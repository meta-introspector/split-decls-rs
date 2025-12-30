// Generated macro for macro_124 (macro)
macro_rules! Depcrate_opcodemacro_124 {
() => {
// Module: crate::opcode
// Provides: {"macro_124"}
// Dependencies: {}
opcode ! { # [doc = " Perform file truncation, equivalent to `ftruncate(2)`."] # [derive (Debug)] pub struct Ftruncate { fd : { impl sealed :: UseFixed } , len : { u64 } , ;; } pub const CODE = sys :: IORING_OP_FTRUNCATE ; pub fn build (self) -> Entry { let Ftruncate { fd , len } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_1 . off = len ; Entry (sqe) } }
};
}

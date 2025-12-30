// Generated macro for macro_81 (macro)
macro_rules! Depcrate_opcodemacro_81 {
() => {
// Module: crate::opcode
// Provides: {"macro_81"}
// Dependencies: {}
opcode ! { # [doc = " Preallocate or deallocate space to a file, equivalent to `fallocate(2)`."] pub struct Fallocate { fd : { impl sealed :: UseFixed } , len : { u64 } , ;; offset : u64 = 0 , mode : i32 = 0 } pub const CODE = sys :: IORING_OP_FALLOCATE ; pub fn build (self) -> Entry { let Fallocate { fd , len , offset , mode } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = len ; sqe . len = mode as _ ; sqe . __bindgen_anon_1 . off = offset ; Entry (sqe) } }
};
}

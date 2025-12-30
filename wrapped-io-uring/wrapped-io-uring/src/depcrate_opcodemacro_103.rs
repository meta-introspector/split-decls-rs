// Generated macro for macro_103 (macro)
macro_rules! Depcrate_opcodemacro_103 {
() => {
// Module: crate::opcode
// Provides: {"macro_103"}
// Dependencies: {}
opcode ! { # [doc = " Create a symlink, equivalent to `symlinkat(2)`."] pub struct SymlinkAt { newdirfd : { impl sealed :: UseFd } , target : { * const libc :: c_char } , linkpath : { * const libc :: c_char } , ;; } pub const CODE = sys :: IORING_OP_SYMLINKAT ; pub fn build (self) -> Entry { let SymlinkAt { newdirfd , target , linkpath } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = newdirfd ; sqe . __bindgen_anon_2 . addr = target as _ ; sqe . __bindgen_anon_1 . addr2 = linkpath as _ ; Entry (sqe) } }
};
}

// Generated macro for macro_89 (macro)
macro_rules! Depcrate_opcodemacro_89 {
() => {
// Module: crate::opcode
// Provides: {"macro_89"}
// Dependencies: {}
opcode ! { # [doc = " Give advice about use of memory, equivalent to `madvise(2)`."] pub struct Madvise { addr : { * const libc :: c_void } , len : { libc :: off_t } , advice : { i32 } , ;; } pub const CODE = sys :: IORING_OP_MADVISE ; pub fn build (self) -> Entry { let Madvise { addr , len , advice } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = addr as _ ; sqe . len = len as _ ; sqe . __bindgen_anon_3 . fadvise_advice = advice as _ ; Entry (sqe) } }
};
}

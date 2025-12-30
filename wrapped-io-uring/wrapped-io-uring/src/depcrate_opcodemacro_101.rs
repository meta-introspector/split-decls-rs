// Generated macro for macro_101 (macro)
macro_rules! Depcrate_opcodemacro_101 {
() => {
// Module: crate::opcode
// Provides: {"macro_101"}
// Dependencies: {}
opcode ! { pub struct UnlinkAt { dirfd : { impl sealed :: UseFd } , pathname : { * const libc :: c_char } , ;; flags : i32 = 0 } pub const CODE = sys :: IORING_OP_UNLINKAT ; pub fn build (self) -> Entry { let UnlinkAt { dirfd , pathname , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = dirfd ; sqe . __bindgen_anon_2 . addr = pathname as _ ; sqe . __bindgen_anon_3 . unlink_flags = flags as _ ; Entry (sqe) } }
};
}

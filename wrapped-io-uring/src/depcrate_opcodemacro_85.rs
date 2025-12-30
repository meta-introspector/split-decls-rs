// Generated macro for macro_85 (macro)
macro_rules! Depcrate_opcodemacro_85 {
() => {
// Module: crate::opcode
// Provides: {"macro_85"}
// Dependencies: {}
opcode ! { # [doc = " Get file status, equivalent to `statx(2)`."] pub struct Statx { dirfd : { impl sealed :: UseFd } , pathname : { * const libc :: c_char } , statxbuf : { * mut types :: statx } , ;; flags : i32 = 0 , mask : u32 = 0 } pub const CODE = sys :: IORING_OP_STATX ; pub fn build (self) -> Entry { let Statx { dirfd , pathname , statxbuf , flags , mask } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = dirfd ; sqe . __bindgen_anon_2 . addr = pathname as _ ; sqe . len = mask ; sqe . __bindgen_anon_1 . off = statxbuf as _ ; sqe . __bindgen_anon_3 . statx_flags = flags as _ ; Entry (sqe) } }
};
}

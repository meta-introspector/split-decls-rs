// Generated macro for macro_108 (macro)
macro_rules! Depcrate_opcodemacro_108 {
() => {
// Module: crate::opcode
// Provides: {"macro_108"}
// Dependencies: {}
opcode ! { # [doc = " Set extended attribute on a file descriptor, equivalent to `fsetxattr(2)`."] pub struct FSetXattr { fd : { impl sealed :: UseFixed } , name : { * const libc :: c_char } , value : { * const libc :: c_void } , len : { u32 } , ;; flags : i32 = 0 } pub const CODE = sys :: IORING_OP_FSETXATTR ; pub fn build (self) -> Entry { let FSetXattr { fd , name , value , flags , len } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = name as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = value as _ ; sqe . __bindgen_anon_3 . xattr_flags = flags as _ ; Entry (sqe) } }
};
}

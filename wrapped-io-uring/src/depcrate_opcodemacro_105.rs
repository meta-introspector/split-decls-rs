// Generated macro for macro_105 (macro)
macro_rules! Depcrate_opcodemacro_105 {
() => {
// Module: crate::opcode
// Provides: {"macro_105"}
// Dependencies: {}
opcode ! { # [doc = " Get extended attribute, equivalent to `getxattr(2)`."] pub struct GetXattr { name : { * const libc :: c_char } , value : { * mut libc :: c_void } , path : { * const libc :: c_char } , len : { u32 } , ;; } pub const CODE = sys :: IORING_OP_GETXATTR ; pub fn build (self) -> Entry { let GetXattr { name , value , path , len } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . __bindgen_anon_2 . addr = name as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = value as _ ; unsafe { sqe . __bindgen_anon_6 . __bindgen_anon_1 . as_mut () . addr3 = path as _ } ; sqe . __bindgen_anon_3 . xattr_flags = 0 ; Entry (sqe) } }
};
}

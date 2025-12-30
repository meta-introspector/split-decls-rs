// Generated macro for macro_114 (macro)
macro_rules! Depcrate_opcodemacro_114 {
() => {
// Module: crate::opcode
// Provides: {"macro_114"}
// Dependencies: {}
opcode ! { # [doc = " Accept multiple new connections on a socket."] # [doc = ""] # [doc = " Set the `allocate_file_index` property if fixed file table entries should be used."] # [doc = ""] # [doc = " Available since 5.19."] pub struct AcceptMulti { fd : { impl sealed :: UseFixed } , ;; allocate_file_index : bool = false , flags : i32 = 0 } pub const CODE = sys :: IORING_OP_ACCEPT ; pub fn build (self) -> Entry { let AcceptMulti { fd , allocate_file_index , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . ioprio = sys :: IORING_ACCEPT_MULTISHOT as u16 ; sqe . __bindgen_anon_3 . accept_flags = flags as _ ; if allocate_file_index { sqe . __bindgen_anon_5 . file_index = sys :: IORING_FILE_INDEX_ALLOC as u32 ; } Entry (sqe) } }
};
}

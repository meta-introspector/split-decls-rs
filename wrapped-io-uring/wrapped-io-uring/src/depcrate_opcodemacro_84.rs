// Generated macro for macro_84 (macro)
macro_rules! Depcrate_opcodemacro_84 {
() => {
// Module: crate::opcode
// Provides: {"macro_84"}
// Dependencies: {}
opcode ! { # [doc = " This command is an alternative to using"] # [doc = " [`Submitter::register_files_update`](crate::Submitter::register_files_update) which then"] # [doc = " works in an async fashion, like the rest of the io_uring commands."] pub struct FilesUpdate { fds : { * const RawFd } , len : { u32 } , ;; offset : i32 = 0 } pub const CODE = sys :: IORING_OP_FILES_UPDATE ; pub fn build (self) -> Entry { let FilesUpdate { fds , len , offset } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = fds as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = offset as _ ; Entry (sqe) } }
};
}

// Generated macro for macro_123 (macro)
macro_rules! Depcrate_opcodemacro_123 {
() => {
// Module: crate::opcode
// Provides: {"macro_123"}
// Dependencies: {}
opcode ! { # [doc = " Install a fixed file descriptor"] # [doc = ""] # [doc = " Turns a direct descriptor into a regular file descriptor that can be later used by regular"] # [doc = " system calls that take a normal raw file descriptor"] # [derive (Debug)] pub struct FixedFdInstall { fd : { types :: Fixed } , file_flags : { u32 } , ;; } pub const CODE = sys :: IORING_OP_FIXED_FD_INSTALL ; pub fn build (self) -> Entry { let FixedFdInstall { fd , file_flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = fd . 0 as _ ; sqe . flags = crate :: squeue :: Flags :: FIXED_FILE . bits () ; sqe . __bindgen_anon_3 . install_fd_flags = file_flags ; Entry (sqe) } }
};
}

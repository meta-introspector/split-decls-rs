// Generated macro for macro_134 (macro)
macro_rules! Depcrate_opcodemacro_134 {
() => {
// Module: crate::opcode
// Provides: {"macro_134"}
// Dependencies: {}
opcode ! { pub struct Pipe { fds : { * mut RawFd } , ;; flags : u32 = 0 , file_index : Option < types :: DestinationSlot > = None , } pub const CODE = sys :: IORING_OP_PIPE ; pub fn build (self) -> Entry { let Self { fds , flags , file_index } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = 0 ; sqe . __bindgen_anon_2 . addr = fds as _ ; sqe . __bindgen_anon_3 . pipe_flags = flags ; if let Some (dest) = file_index { sqe . __bindgen_anon_5 . file_index = dest . kernel_index_arg () ; } Entry (sqe) } }
};
}

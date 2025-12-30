// Generated macro for macro_97 (macro)
macro_rules! Depcrate_opcodemacro_97 {
() => {
// Module: crate::opcode
// Provides: {"macro_97"}
// Dependencies: {}
opcode ! { # [doc = " Remove some number of buffers from a buffer group. See"] # [doc = " [`BUFFER_SELECT`](crate::squeue::Flags::BUFFER_SELECT) for more info."] pub struct RemoveBuffers { nbufs : { u16 } , bgid : { u16 } ;; } pub const CODE = sys :: IORING_OP_REMOVE_BUFFERS ; pub fn build (self) -> Entry { let RemoveBuffers { nbufs , bgid } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = nbufs as _ ; sqe . __bindgen_anon_4 . buf_group = bgid ; Entry (sqe) } }
};
}

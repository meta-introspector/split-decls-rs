// Generated macro for macro_115 (macro)
macro_rules! Depcrate_opcodemacro_115 {
() => {
// Module: crate::opcode
// Provides: {"macro_115"}
// Dependencies: {}
opcode ! { # [doc = " Send a message (with fixed FD) to a target ring."] pub struct MsgRingSendFd { ring_fd : { impl sealed :: UseFd } , fixed_slot_src : { types :: Fixed } , dest_slot_index : { types :: DestinationSlot } , user_data : { u64 } , ;; opcode_flags : u32 = 0 } pub const CODE = sys :: IORING_OP_MSG_RING ; pub fn build (self) -> Entry { let MsgRingSendFd { ring_fd , fixed_slot_src , dest_slot_index , user_data , opcode_flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . __bindgen_anon_2 . addr = sys :: IORING_MSG_SEND_FD . into () ; sqe . fd = ring_fd ; sqe . __bindgen_anon_1 . off = user_data ; unsafe { sqe . __bindgen_anon_6 . __bindgen_anon_1 . as_mut () . addr3 = fixed_slot_src . 0 as u64 } ; sqe . __bindgen_anon_5 . file_index = dest_slot_index . kernel_index_arg () ; sqe . __bindgen_anon_3 . msg_ring_flags = opcode_flags ; Entry (sqe) } }
};
}

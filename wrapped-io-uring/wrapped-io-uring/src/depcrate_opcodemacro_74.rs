// Generated macro for macro_74 (macro)
macro_rules! Depcrate_opcodemacro_74 {
() => {
// Module: crate::opcode
// Provides: {"macro_74"}
// Dependencies: {}
opcode ! { # [doc = " Attempt to remove an existing [timeout operation](Timeout)."] pub struct TimeoutRemove { user_data : { u64 } , ;; } pub const CODE = sys :: IORING_OP_TIMEOUT_REMOVE ; pub fn build (self) -> Entry { let TimeoutRemove { user_data } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = user_data ; Entry (sqe) } }
};
}

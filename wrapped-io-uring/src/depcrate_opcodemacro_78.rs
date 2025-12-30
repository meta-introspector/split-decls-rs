// Generated macro for macro_78 (macro)
macro_rules! Depcrate_opcodemacro_78 {
() => {
// Module: crate::opcode
// Provides: {"macro_78"}
// Dependencies: {}
opcode ! { # [doc = " Attempt to cancel an already issued request."] pub struct AsyncCancel { user_data : { u64 } ;; } pub const CODE = sys :: IORING_OP_ASYNC_CANCEL ; pub fn build (self) -> Entry { let AsyncCancel { user_data } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = user_data ; Entry (sqe) } }
};
}

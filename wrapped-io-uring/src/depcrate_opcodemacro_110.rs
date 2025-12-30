// Generated macro for macro_110 (macro)
macro_rules! Depcrate_opcodemacro_110 {
() => {
// Module: crate::opcode
// Provides: {"macro_110"}
// Dependencies: {}
opcode ! { # [doc = " Attempt to cancel an already issued request, receiving a cancellation"] # [doc = " builder, which allows for the new cancel criterias introduced since"] # [doc = " 5.19."] pub struct AsyncCancel2 { builder : { types :: CancelBuilder } ;; } pub const CODE = sys :: IORING_OP_ASYNC_CANCEL ; pub fn build (self) -> Entry { let AsyncCancel2 { builder } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = builder . to_fd () ; sqe . __bindgen_anon_2 . addr = builder . user_data . unwrap_or (0) ; sqe . __bindgen_anon_3 . cancel_flags = builder . flags . bits () ; Entry (sqe) } }
};
}

// Generated macro for macro_79 (macro)
macro_rules! Depcrate_opcodemacro_79 {
() => {
// Module: crate::opcode
// Provides: {"macro_79"}
// Dependencies: {}
opcode ! { # [doc = " This request must be linked with another request through"] # [doc = " [`Flags::IO_LINK`](crate::squeue::Flags::IO_LINK) which is described below."] # [doc = " Unlike [`Timeout`], [`LinkTimeout`] acts on the linked request, not the completion queue."] pub struct LinkTimeout { timespec : { * const types :: Timespec } , ;; flags : types :: TimeoutFlags = types :: TimeoutFlags :: empty () } pub const CODE = sys :: IORING_OP_LINK_TIMEOUT ; pub fn build (self) -> Entry { let LinkTimeout { timespec , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; sqe . __bindgen_anon_2 . addr = timespec as _ ; sqe . len = 1 ; sqe . __bindgen_anon_3 . timeout_flags = flags . bits () ; Entry (sqe) } }
};
}

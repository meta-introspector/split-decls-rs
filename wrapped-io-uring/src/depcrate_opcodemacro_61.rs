// Generated macro for macro_61 (macro)
macro_rules! Depcrate_opcodemacro_61 {
() => {
// Module: crate::opcode
// Provides: {"macro_61"}
// Dependencies: {}
opcode ! { # [doc = " Do not perform any I/O."] # [doc = ""] # [doc = " This is useful for testing the performance of the io_uring implementation itself."] # [derive (Debug)] pub struct Nop { ;; } pub const CODE = sys :: IORING_OP_NOP ; pub fn build (self) -> Entry { let Nop { } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; Entry (sqe) } }
};
}

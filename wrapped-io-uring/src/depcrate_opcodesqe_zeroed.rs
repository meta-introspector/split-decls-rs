// Generated macro for sqe_zeroed (function)
macro_rules! Depcrate_opcodesqe_zeroed {
() => {
// Module: crate::opcode
// Provides: {"sqe_zeroed"}
// Dependencies: {}
# [doc = " inline zeroed to improve codegen"] # [inline (always)] fn sqe_zeroed () -> sys :: io_uring_sqe { unsafe { mem :: zeroed () } }
};
}

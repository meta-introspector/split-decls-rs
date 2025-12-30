// Generated macro for get_clobbered_fprs (function)
macro_rules! Depcrate_isa_s390x_abiget_clobbered_fprs {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"get_clobbered_fprs"}
// Dependencies: {}
fn get_clobbered_fprs (frame_layout : & FrameLayout) -> & [Writable < RealReg >] { let (_ , clobbered_fpr) = frame_layout . clobbered_callee_saves_by_class () ; clobbered_fpr }
};
}

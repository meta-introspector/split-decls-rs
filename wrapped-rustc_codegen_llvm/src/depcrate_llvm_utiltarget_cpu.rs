// Generated macro for target_cpu (function)
macro_rules! Depcrate_llvm_utiltarget_cpu {
() => {
// Module: crate::llvm_util
// Provides: {"target_cpu"}
// Dependencies: {}
pub (crate) fn target_cpu (sess : & Session) -> & str { let cpu_name = sess . opts . cg . target_cpu . as_deref () . unwrap_or_else (| | & sess . target . cpu) ; handle_native (cpu_name) }
};
}

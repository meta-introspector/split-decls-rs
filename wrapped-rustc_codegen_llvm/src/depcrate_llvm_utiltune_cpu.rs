// Generated macro for tune_cpu (function)
macro_rules! Depcrate_llvm_utiltune_cpu {
() => {
// Module: crate::llvm_util
// Provides: {"tune_cpu"}
// Dependencies: {}
pub (crate) fn tune_cpu (sess : & Session) -> Option < & str > { let name = sess . opts . unstable_opts . tune_cpu . as_ref () ? ; Some (handle_native (name)) }
};
}

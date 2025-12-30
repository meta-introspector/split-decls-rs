// Generated macro for require_inited (function)
macro_rules! Depcrate_llvm_utilrequire_inited {
() => {
// Module: crate::llvm_util
// Provides: {"require_inited"}
// Dependencies: {}
fn require_inited () { if ! INIT . is_completed () { bug ! ("LLVM is not initialized") ; } }
};
}

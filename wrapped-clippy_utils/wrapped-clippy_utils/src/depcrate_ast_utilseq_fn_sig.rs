// Generated macro for eq_fn_sig (function)
macro_rules! Depcrate_ast_utilseq_fn_sig {
() => {
// Module: crate::ast_utils
// Provides: {"eq_fn_sig"}
// Dependencies: {}
pub fn eq_fn_sig (l : & FnSig , r : & FnSig) -> bool { eq_fn_decl (& l . decl , & r . decl) && eq_fn_header (& l . header , & r . header) }
};
}

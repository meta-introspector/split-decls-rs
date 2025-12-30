// Generated macro for eq_qself (function)
macro_rules! Depcrate_ast_utilseq_qself {
() => {
// Module: crate::ast_utils
// Provides: {"eq_qself"}
// Dependencies: {}
pub fn eq_qself (l : & QSelf , r : & QSelf) -> bool { l . position == r . position && eq_ty (& l . ty , & r . ty) }
};
}

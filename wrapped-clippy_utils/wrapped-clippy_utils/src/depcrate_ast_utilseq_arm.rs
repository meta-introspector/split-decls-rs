// Generated macro for eq_arm (function)
macro_rules! Depcrate_ast_utilseq_arm {
() => {
// Module: crate::ast_utils
// Provides: {"eq_arm"}
// Dependencies: {}
pub fn eq_arm (l : & Arm , r : & Arm) -> bool { l . is_placeholder == r . is_placeholder && eq_pat (& l . pat , & r . pat) && eq_expr_opt (l . body . as_deref () , r . body . as_deref ()) && eq_expr_opt (l . guard . as_deref () , r . guard . as_deref ()) && over (& l . attrs , & r . attrs , eq_attr) }
};
}

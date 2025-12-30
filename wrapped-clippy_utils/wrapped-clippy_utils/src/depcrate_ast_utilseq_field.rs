// Generated macro for eq_field (function)
macro_rules! Depcrate_ast_utilseq_field {
() => {
// Module: crate::ast_utils
// Provides: {"eq_field"}
// Dependencies: {}
pub fn eq_field (l : & ExprField , r : & ExprField) -> bool { l . is_placeholder == r . is_placeholder && eq_id (l . ident , r . ident) && eq_expr (& l . expr , & r . expr) && over (& l . attrs , & r . attrs , eq_attr) }
};
}

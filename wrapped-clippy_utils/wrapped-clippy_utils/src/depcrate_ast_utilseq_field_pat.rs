// Generated macro for eq_field_pat (function)
macro_rules! Depcrate_ast_utilseq_field_pat {
() => {
// Module: crate::ast_utils
// Provides: {"eq_field_pat"}
// Dependencies: {}
pub fn eq_field_pat (l : & PatField , r : & PatField) -> bool { l . is_placeholder == r . is_placeholder && eq_id (l . ident , r . ident) && eq_pat (& l . pat , & r . pat) && over (& l . attrs , & r . attrs , eq_attr) }
};
}

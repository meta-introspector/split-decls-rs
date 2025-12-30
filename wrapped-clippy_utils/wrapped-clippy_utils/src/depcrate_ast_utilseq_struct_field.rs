// Generated macro for eq_struct_field (function)
macro_rules! Depcrate_ast_utilseq_struct_field {
() => {
// Module: crate::ast_utils
// Provides: {"eq_struct_field"}
// Dependencies: {}
pub fn eq_struct_field (l : & FieldDef , r : & FieldDef) -> bool { l . is_placeholder == r . is_placeholder && over (& l . attrs , & r . attrs , eq_attr) && eq_vis (& l . vis , & r . vis) && both (l . ident . as_ref () , r . ident . as_ref () , | l , r | eq_id (* l , * r)) && eq_ty (& l . ty , & r . ty) }
};
}

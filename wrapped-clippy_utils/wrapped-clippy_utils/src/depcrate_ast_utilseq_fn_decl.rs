// Generated macro for eq_fn_decl (function)
macro_rules! Depcrate_ast_utilseq_fn_decl {
() => {
// Module: crate::ast_utils
// Provides: {"eq_fn_decl"}
// Dependencies: {}
pub fn eq_fn_decl (l : & FnDecl , r : & FnDecl) -> bool { eq_fn_ret_ty (& l . output , & r . output) && over (& l . inputs , & r . inputs , | l , r | { l . is_placeholder == r . is_placeholder && eq_pat (& l . pat , & r . pat) && eq_ty (& l . ty , & r . ty) && over (& l . attrs , & r . attrs , eq_attr) }) }
};
}

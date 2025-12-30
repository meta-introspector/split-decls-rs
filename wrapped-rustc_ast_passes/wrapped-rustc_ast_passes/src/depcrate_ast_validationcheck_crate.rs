// Generated macro for check_crate (function)
macro_rules! Depcrate_ast_validationcheck_crate {
() => {
// Module: crate::ast_validation
// Provides: {"check_crate"}
// Dependencies: {}
pub fn check_crate (sess : & Session , features : & Features , krate : & Crate , is_sdylib_interface : bool , lints : & mut LintBuffer ,) -> bool { let mut validator = AstValidator { sess , features , extern_mod_span : None , outer_trait_or_trait_impl : None , has_proc_macro_decls : false , outer_impl_trait_span : None , disallow_tilde_const : Some (TildeConstReason :: Item) , extern_mod_safety : None , extern_mod_abi : None , lint_node_id : CRATE_NODE_ID , is_sdylib_interface , lint_buffer : lints , } ; visit :: walk_crate (& mut validator , krate) ; validator . has_proc_macro_decls }
};
}

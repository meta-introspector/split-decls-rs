// Generated macro for eq_opt_fn_contract (function)
macro_rules! Depcrate_ast_utilseq_opt_fn_contract {
() => {
// Module: crate::ast_utils
// Provides: {"eq_opt_fn_contract"}
// Dependencies: {}
# [expect (clippy :: ref_option , reason = "This is the type how it is stored in the AST")] pub fn eq_opt_fn_contract (l : & Option < Box < FnContract > > , r : & Option < Box < FnContract > >) -> bool { match (l , r) { (Some (l) , Some (r)) => { eq_expr_opt (l . requires . as_deref () , r . requires . as_deref ()) && eq_expr_opt (l . ensures . as_deref () , r . ensures . as_deref ()) } , (None , None) => true , (Some (_) , None) | (None , Some (_)) => false , } }
};
}

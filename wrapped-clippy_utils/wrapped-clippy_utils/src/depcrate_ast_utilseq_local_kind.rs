// Generated macro for eq_local_kind (function)
macro_rules! Depcrate_ast_utilseq_local_kind {
() => {
// Module: crate::ast_utils
// Provides: {"eq_local_kind"}
// Dependencies: {}
pub fn eq_local_kind (l : & LocalKind , r : & LocalKind) -> bool { use LocalKind :: * ; match (l , r) { (Decl , Decl) => true , (Init (l) , Init (r)) => eq_expr (l , r) , (InitElse (li , le) , InitElse (ri , re)) => eq_expr (li , ri) && eq_block (le , re) , _ => false , } }
};
}

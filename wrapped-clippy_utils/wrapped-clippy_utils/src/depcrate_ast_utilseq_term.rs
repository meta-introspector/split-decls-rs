// Generated macro for eq_term (function)
macro_rules! Depcrate_ast_utilseq_term {
() => {
// Module: crate::ast_utils
// Provides: {"eq_term"}
// Dependencies: {}
fn eq_term (l : & Term , r : & Term) -> bool { match (l , r) { (Term :: Ty (l) , Term :: Ty (r)) => eq_ty (l , r) , (Term :: Const (l) , Term :: Const (r)) => eq_anon_const (l , r) , _ => false , } }
};
}

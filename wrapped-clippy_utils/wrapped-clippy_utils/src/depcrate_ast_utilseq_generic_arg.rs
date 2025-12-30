// Generated macro for eq_generic_arg (function)
macro_rules! Depcrate_ast_utilseq_generic_arg {
() => {
// Module: crate::ast_utils
// Provides: {"eq_generic_arg"}
// Dependencies: {}
pub fn eq_generic_arg (l : & GenericArg , r : & GenericArg) -> bool { match (l , r) { (GenericArg :: Lifetime (l) , GenericArg :: Lifetime (r)) => eq_id (l . ident , r . ident) , (GenericArg :: Type (l) , GenericArg :: Type (r)) => eq_ty (l , r) , (GenericArg :: Const (l) , GenericArg :: Const (r)) => eq_expr (& l . value , & r . value) , _ => false , } }
};
}

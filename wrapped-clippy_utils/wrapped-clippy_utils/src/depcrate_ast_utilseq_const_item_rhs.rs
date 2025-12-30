// Generated macro for eq_const_item_rhs (function)
macro_rules! Depcrate_ast_utilseq_const_item_rhs {
() => {
// Module: crate::ast_utils
// Provides: {"eq_const_item_rhs"}
// Dependencies: {}
pub fn eq_const_item_rhs (l : & ConstItemRhs , r : & ConstItemRhs) -> bool { use ConstItemRhs :: * ; match (l , r) { (TypeConst (l) , TypeConst (r)) => eq_anon_const (l , r) , (Body (l) , Body (r)) => eq_expr (l , r) , (TypeConst (..) , Body (..)) | (Body (..) , TypeConst (..)) => false , } }
};
}

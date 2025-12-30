// Generated macro for eq_use_tree_kind (function)
macro_rules! Depcrate_ast_utilseq_use_tree_kind {
() => {
// Module: crate::ast_utils
// Provides: {"eq_use_tree_kind"}
// Dependencies: {}
pub fn eq_use_tree_kind (l : & UseTreeKind , r : & UseTreeKind) -> bool { use UseTreeKind :: * ; match (l , r) { (Glob , Glob) => true , (Simple (l) , Simple (r)) => both (l . as_ref () , r . as_ref () , | l , r | eq_id (* l , * r)) , (Nested { items : l , .. } , Nested { items : r , .. }) => over (l , r , | (l , _) , (r , _) | eq_use_tree (l , r)) , _ => false , } }
};
}

// Generated macro for eq_assoc_item_constraint (function)
macro_rules! Depcrate_ast_utilseq_assoc_item_constraint {
() => {
// Module: crate::ast_utils
// Provides: {"eq_assoc_item_constraint"}
// Dependencies: {}
pub fn eq_assoc_item_constraint (l : & AssocItemConstraint , r : & AssocItemConstraint) -> bool { use AssocItemConstraintKind :: * ; eq_id (l . ident , r . ident) && match (& l . kind , & r . kind) { (Equality { term : l } , Equality { term : r }) => eq_term (l , r) , (Bound { bounds : l } , Bound { bounds : r }) => over (l , r , eq_generic_bound) , _ => false , } }
};
}

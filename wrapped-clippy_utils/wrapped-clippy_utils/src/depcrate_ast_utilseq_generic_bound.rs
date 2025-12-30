// Generated macro for eq_generic_bound (function)
macro_rules! Depcrate_ast_utilseq_generic_bound {
() => {
// Module: crate::ast_utils
// Provides: {"eq_generic_bound"}
// Dependencies: {}
pub fn eq_generic_bound (l : & GenericBound , r : & GenericBound) -> bool { use GenericBound :: * ; match (l , r) { (Trait (ptr1) , Trait (ptr2)) => eq_poly_ref_trait (ptr1 , ptr2) , (Outlives (l) , Outlives (r)) => eq_id (l . ident , r . ident) , _ => false , } }
};
}

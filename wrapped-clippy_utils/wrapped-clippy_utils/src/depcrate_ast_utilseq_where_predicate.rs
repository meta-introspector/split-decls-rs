// Generated macro for eq_where_predicate (function)
macro_rules! Depcrate_ast_utilseq_where_predicate {
() => {
// Module: crate::ast_utils
// Provides: {"eq_where_predicate"}
// Dependencies: {}
pub fn eq_where_predicate (l : & WherePredicate , r : & WherePredicate) -> bool { use WherePredicateKind :: * ; over (& l . attrs , & r . attrs , eq_attr) && match (& l . kind , & r . kind) { (BoundPredicate (l) , BoundPredicate (r)) => { over (& l . bound_generic_params , & r . bound_generic_params , | l , r | { eq_generic_param (l , r) }) && eq_ty (& l . bounded_ty , & r . bounded_ty) && over (& l . bounds , & r . bounds , eq_generic_bound) } , (RegionPredicate (l) , RegionPredicate (r)) => { eq_id (l . lifetime . ident , r . lifetime . ident) && over (& l . bounds , & r . bounds , eq_generic_bound) } , (EqPredicate (l) , EqPredicate (r)) => eq_ty (& l . lhs_ty , & r . lhs_ty) && eq_ty (& l . rhs_ty , & r . rhs_ty) , _ => false , } }
};
}

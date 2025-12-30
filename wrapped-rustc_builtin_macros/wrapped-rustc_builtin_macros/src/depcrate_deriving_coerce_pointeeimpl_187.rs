// Generated macro for impl_187 (impl)
macro_rules! Depcrate_deriving_coerce_pointeeimpl_187 {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'a > ast :: mut_visit :: MutVisitor for TypeSubstitution < 'a > { fn visit_ty (& mut self , ty : & mut ast :: Ty) { if let Some (name) = ty . kind . is_simple_path () && name == self . from_name { * ty = self . to_ty . clone () ; self . rewritten = true ; } else { ast :: mut_visit :: walk_ty (self , ty) ; } } fn visit_where_predicate_kind (& mut self , kind : & mut ast :: WherePredicateKind) { match kind { rustc_ast :: WherePredicateKind :: BoundPredicate (bound) => { bound . bound_generic_params . flat_map_in_place (| param | self . flat_map_generic_param (param)) ; self . visit_ty (& mut bound . bounded_ty) ; for bound in & mut bound . bounds { self . visit_param_bound (bound , BoundKind :: Bound) } } rustc_ast :: WherePredicateKind :: RegionPredicate (_) | rustc_ast :: WherePredicateKind :: EqPredicate (_) => { } } } }
};
}

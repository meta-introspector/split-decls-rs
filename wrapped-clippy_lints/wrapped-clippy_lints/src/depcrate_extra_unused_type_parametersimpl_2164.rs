// Generated macro for impl_2164 (impl)
macro_rules! Depcrate_extra_unused_type_parametersimpl_2164 {
() => {
// Module: crate::extra_unused_type_parameters
// Provides: {"impl_2164"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for TypeWalker < '_ , 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn visit_ty (& mut self , t : & 'tcx Ty < 'tcx , AmbigArg >) { if let Some ((def_id , _)) = t . peel_refs () . as_generic_param () { self . ty_params . remove (& def_id) ; } else { walk_ty (self , t) ; } } fn visit_where_predicate (& mut self , predicate : & 'tcx WherePredicate < 'tcx >) { let span = predicate . span ; if let WherePredicateKind :: BoundPredicate (predicate) = predicate . kind { if let Some ((def_id , _)) = predicate . bounded_ty . peel_refs () . as_generic_param () { match predicate . origin { PredicateOrigin :: GenericParam => { self . inline_bounds . insert (def_id , span) ; } , PredicateOrigin :: WhereClause => { self . where_bounds . insert (def_id) ; } , PredicateOrigin :: ImplTrait => () , } if ! predicate . bounds . iter () . filter_map (bound_to_trait_def_id) . all (| id | self . cx . effective_visibilities . is_exported (id)) { self . ty_params . remove (& def_id) ; } } else { walk_unambig_ty (self , predicate . bounded_ty) ; } for bound in predicate . bounds { walk_param_bound (self , bound) ; } } } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}

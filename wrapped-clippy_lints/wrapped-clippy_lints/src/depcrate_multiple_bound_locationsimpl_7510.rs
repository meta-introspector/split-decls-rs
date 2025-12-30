// Generated macro for impl_7510 (impl)
macro_rules! Depcrate_multiple_bound_locationsimpl_7510 {
() => {
// Module: crate::multiple_bound_locations
// Provides: {"impl_7510"}
// Dependencies: {}
impl EarlyLintPass for MultipleBoundLocations { fn check_fn (& mut self , cx : & EarlyContext < '_ > , kind : FnKind < '_ > , _ : Span , _ : NodeId) { if let FnKind :: Fn (_ , _ , Fn { generics , .. }) = kind && ! generics . params . is_empty () && ! generics . where_clause . predicates . is_empty () { let mut generic_params_with_bounds = FxHashMap :: default () ; for param in & generics . params { if ! param . bounds . is_empty () { generic_params_with_bounds . insert (param . ident . as_str () , param . ident . span) ; } } for clause in & generics . where_clause . predicates { match & clause . kind { WherePredicateKind :: BoundPredicate (pred) => { if (! pred . bound_generic_params . is_empty () || ! pred . bounds . is_empty ()) && let Some (Some (bound_span)) = pred . bounded_ty . span . with_source_text (cx , | src | generic_params_with_bounds . get (src)) { emit_lint (cx , * bound_span , pred . bounded_ty . span) ; } } , WherePredicateKind :: RegionPredicate (pred) => { if ! pred . bounds . is_empty () && let Some (bound_span) = generic_params_with_bounds . get (& pred . lifetime . ident . as_str ()) { emit_lint (cx , * bound_span , pred . lifetime . ident . span) ; } } , WherePredicateKind :: EqPredicate (_) => { } , } } } } }
};
}

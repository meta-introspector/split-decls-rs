// Generated macro for impl_3441 (impl)
macro_rules! Depcrate_lifetimesimpl_3441 {
() => {
// Module: crate::lifetimes
// Provides: {"impl_3441"}
// Dependencies: {}
impl < 'tcx , F > Visitor < 'tcx > for LifetimeChecker < '_ , 'tcx , F > where F : NestedFilter < 'tcx > , { type MaybeTyCtxt = TyCtxt < 'tcx > ; type NestedFilter = F ; fn visit_lifetime (& mut self , lifetime : & 'tcx Lifetime) { if let LifetimeKind :: Param (def_id) = lifetime . kind && let Some (usages) = self . map . get_mut (& def_id) { usages . push (Usage { lifetime : * lifetime , in_where_predicate : self . where_predicate_depth != 0 , in_bounded_ty : self . bounded_ty_depth != 0 , in_generics_arg : self . generic_args_depth != 0 , lifetime_elision_impossible : self . lifetime_elision_impossible , }) ; } } fn visit_where_predicate (& mut self , predicate : & 'tcx WherePredicate < 'tcx >) { self . where_predicate_depth += 1 ; if let & WherePredicateKind :: BoundPredicate (WhereBoundPredicate { bounded_ty , bounds , bound_generic_params , origin : _ , }) = predicate . kind { self . visit_where_bound_predicate (predicate . hir_id , bounded_ty , bounds , bound_generic_params) ; } else { walk_where_predicate (self , predicate) ; } self . where_predicate_depth -= 1 ; } fn visit_generic_args (& mut self , generic_args : & 'tcx GenericArgs < 'tcx >) -> Self :: Result { self . generic_args_depth += 1 ; walk_generic_args (self , generic_args) ; self . generic_args_depth -= 1 ; } fn visit_fn_decl (& mut self , fd : & 'tcx FnDecl < 'tcx >) -> Self :: Result { self . lifetime_elision_impossible = ! is_candidate_for_elision (fd) ; walk_fn_decl (self , fd) ; self . lifetime_elision_impossible = false ; } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}

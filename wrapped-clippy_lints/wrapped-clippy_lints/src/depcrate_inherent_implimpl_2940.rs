// Generated macro for impl_2940 (impl)
macro_rules! Depcrate_inherent_implimpl_2940 {
() => {
// Module: crate::inherent_impl
// Provides: {"impl_2940"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MultipleInherentImpl { fn check_crate_post (& mut self , cx : & LateContext < 'tcx >) { let mut type_map = FxHashMap :: default () ; let mut lint_spans = Vec :: new () ; let (impls , _) = cx . tcx . crate_inherent_impls (()) ; for (& id , impl_ids) in & impls . inherent_impls { if impl_ids . len () < 2 || is_lint_allowed (cx , MULTIPLE_INHERENT_IMPL , cx . tcx . local_def_id_to_hir_id (id) ,) { continue ; } for impl_id in impl_ids . iter () . map (| id | id . expect_local ()) { let impl_ty = cx . tcx . type_of (impl_id) . instantiate_identity () ; match type_map . entry (impl_ty) { Entry :: Vacant (e) => { e . insert (IdOrSpan :: Id (impl_id)) ; } , Entry :: Occupied (mut e) => { if let Some (span) = get_impl_span (cx , impl_id) { let first_span = match * e . get () { IdOrSpan :: Span (s) => s , IdOrSpan :: Id (id) => { if let Some (s) = get_impl_span (cx , id) { * e . get_mut () = IdOrSpan :: Span (s) ; s } else { * e . get_mut () = IdOrSpan :: Span (span) ; continue ; } } , } ; lint_spans . push ((span , first_span)) ; } } , } } type_map . clear () ; } lint_spans . sort_by_key (| x | x . 0 . lo ()) ; for (span , first_span) in lint_spans { span_lint_and_then (cx , MULTIPLE_INHERENT_IMPL , span , "multiple implementations of this structure" , | diag | { diag . span_note (first_span , "first implementation here") ; } ,) ; } } }
};
}

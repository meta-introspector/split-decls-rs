// Generated macro for impl_3204 (impl)
macro_rules! Depcrate_large_const_arraysimpl_3204 {
() => {
// Module: crate::large_const_arrays
// Provides: {"impl_3204"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for LargeConstArrays { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Const (ident , generics , _ , _) = & item . kind && generics . params . is_empty () && ! generics . has_where_clause_predicates && ! item . span . from_expansion () && let ty = cx . tcx . type_of (item . owner_id) . instantiate_identity () && let ty :: Array (element_type , cst) = ty . kind () && let Some (element_count) = cx . tcx . try_normalize_erasing_regions (cx . typing_env () , * cst) . unwrap_or (* cst) . try_to_target_usize (cx . tcx) && let Ok (element_size) = cx . layout_of (* element_type) . map (| l | l . size . bytes ()) && u128 :: from (self . maximum_allowed_size) < u128 :: from (element_count) * u128 :: from (element_size) { let hi_pos = ident . span . lo () - BytePos :: from_usize (1) ; let sugg_span = Span :: new (hi_pos - BytePos :: from_usize ("const" . len ()) , hi_pos , item . span . ctxt () , item . span . parent () ,) ; span_lint_and_then (cx , LARGE_CONST_ARRAYS , item . span , "large array defined as const" , | diag | { diag . span_suggestion (sugg_span , "make this a static item" , "static" , Applicability :: MachineApplicable ,) ; } ,) ; } } }
};
}

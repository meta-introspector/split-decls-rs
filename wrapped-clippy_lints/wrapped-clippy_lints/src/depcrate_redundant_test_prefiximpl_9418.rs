// Generated macro for impl_9418 (impl)
macro_rules! Depcrate_redundant_test_prefiximpl_9418 {
() => {
// Module: crate::redundant_test_prefix
// Provides: {"impl_9418"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for RedundantTestPrefix { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < '_ > , _decl : & FnDecl < '_ > , body : & 'tcx Body < '_ > , _span : Span , fn_def_id : LocalDefId ,) { let FnKind :: ItemFn (ref ident , ..) = kind else { return ; } ; if ident . span . from_expansion () { return ; } if ! ident . as_str () . starts_with ("test_") { return ; } if ! is_test_function (cx . tcx , fn_def_id) { return ; } span_lint_and_then (cx , REDUNDANT_TEST_PREFIX , ident . span , "redundant `test_` prefix in test function name" , | diag | { let non_prefixed = Symbol :: intern (ident . as_str () . trim_start_matches ("test_")) ; if is_invalid_ident (non_prefixed) { diag . help ("consider function renaming (just removing `test_` prefix will produce invalid function name)" ,) ; } else { let (sugg , msg) : (Cow < '_ , str > , _) = if name_conflicts (cx , body , non_prefixed) { (format ! ("{non_prefixed}_works") . into () , "consider function renaming (just removing `test_` prefix will cause a name conflict)" ,) } else { (non_prefixed . as_str () . into () , "consider removing the `test_` prefix") } ; diag . span_suggestion (ident . span , msg , sugg , Applicability :: MaybeIncorrect) ; } } ,) ; } }
};
}

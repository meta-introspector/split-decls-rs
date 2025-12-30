// Generated macro for check_subpatterns (function)
macro_rules! Depcrate_needless_borrowed_refcheck_subpatterns {
() => {
// Module: crate::needless_borrowed_ref
// Provides: {"check_subpatterns"}
// Dependencies: {}
fn check_subpatterns < 'tcx > (cx : & LateContext < 'tcx > , message : & 'static str , ref_pat : & Pat < '_ > , pat : & Pat < '_ > , subpatterns : impl IntoIterator < Item = & 'tcx Pat < 'tcx > > ,) { let mut suggestions = Vec :: new () ; for subpattern in subpatterns { match subpattern . kind { PatKind :: Binding (BindingMode :: REF , _ , ident , None) => { let span = subpattern . span . until (ident . span) ; suggestions . push ((span , String :: new ())) ; } , PatKind :: Wild => { } , _ => return , } } if ! suggestions . is_empty () { span_lint_and_then (cx , NEEDLESS_BORROWED_REFERENCE , ref_pat . span , message , | diag | { let span = ref_pat . span . until (pat . span) ; suggestions . push ((span , String :: new ())) ; diag . multipart_suggestion ("try removing the `&` and `ref` parts" , suggestions , Applicability :: MachineApplicable ,) ; }) ; } }
};
}

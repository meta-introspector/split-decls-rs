// Generated macro for report_extra_lifetimes (function)
macro_rules! Depcrate_lifetimesreport_extra_lifetimes {
() => {
// Module: crate::lifetimes
// Provides: {"report_extra_lifetimes"}
// Dependencies: {}
fn report_extra_lifetimes < 'tcx > (cx : & LateContext < 'tcx > , func : & 'tcx FnDecl < '_ > , generics : & 'tcx Generics < '_ >) { let mut checker = LifetimeChecker :: < hir_nested_filter :: None > :: new (cx , generics) ; walk_generics (& mut checker , generics) ; walk_fn_decl (& mut checker , func) ; for (def_id , usages) in checker . map { if usages . iter () . all (| usage | usage . in_where_predicate && ! usage . in_bounded_ty && ! usage . in_generics_arg) { span_lint (cx , EXTRA_UNUSED_LIFETIMES , cx . tcx . def_span (def_id) , "this lifetime isn't used in the function definition" ,) ; } } }
};
}

// Generated macro for impl_1580 (impl)
macro_rules! Depcrate_disallowed_namesimpl_1580 {
() => {
// Module: crate::disallowed_names
// Provides: {"impl_1580"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DisallowedNames { fn check_pat (& mut self , cx : & LateContext < 'tcx > , pat : & 'tcx Pat < '_ >) { if let PatKind :: Binding (.. , ident , _) = pat . kind && ! ident . span . from_expansion () && self . disallow . contains (& ident . name) && ! is_in_test (cx . tcx , pat . hir_id) && ! is_from_proc_macro (cx , & ident) { span_lint (cx , DISALLOWED_NAMES , ident . span , format ! ("use of a disallowed/placeholder name `{}`" , ident . name) ,) ; } } }
};
}

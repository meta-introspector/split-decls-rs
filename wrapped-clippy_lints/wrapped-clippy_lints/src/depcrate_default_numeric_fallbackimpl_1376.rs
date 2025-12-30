// Generated macro for impl_1376 (impl)
macro_rules! Depcrate_default_numeric_fallbackimpl_1376 {
() => {
// Module: crate::default_numeric_fallback
// Provides: {"impl_1376"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DefaultNumericFallback { fn check_body (& mut self , cx : & LateContext < 'tcx > , body : & Body < 'tcx >) { let is_parent_const = matches ! (cx . tcx . hir_body_const_context (cx . tcx . hir_body_owner_def_id (body . id ())) , Some (ConstContext :: Const { inline : false } | ConstContext :: Static (_))) ; let mut visitor = NumericFallbackVisitor :: new (cx , is_parent_const) ; visitor . visit_body (body) ; } }
};
}

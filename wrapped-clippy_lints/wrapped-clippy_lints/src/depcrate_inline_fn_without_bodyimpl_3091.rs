// Generated macro for impl_3091 (impl)
macro_rules! Depcrate_inline_fn_without_bodyimpl_3091 {
() => {
// Module: crate::inline_fn_without_body
// Provides: {"impl_3091"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for InlineFnWithoutBody { fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx TraitItem < '_ >) { if let TraitItemKind :: Fn (_ , TraitFn :: Required (_)) = item . kind && let Some (attr_span) = find_attr ! (cx . tcx . hir_attrs (item . hir_id ()) , AttributeKind :: Inline (_ , span) => * span) { span_lint_and_then (cx , INLINE_FN_WITHOUT_BODY , attr_span , format ! ("use of `#[inline]` on trait method `{}` which has no body" , item . ident) , | diag | { diag . suggest_remove_item (cx , attr_span , "remove" , Applicability :: MachineApplicable) ; } ,) ; } } }
};
}

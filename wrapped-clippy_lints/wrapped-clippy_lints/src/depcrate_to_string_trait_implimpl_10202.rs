// Generated macro for impl_10202 (impl)
macro_rules! Depcrate_to_string_trait_implimpl_10202 {
() => {
// Module: crate::to_string_trait_impl
// Provides: {"impl_10202"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ToStringTraitImpl { fn check_item (& mut self , cx : & LateContext < 'tcx > , it : & 'tcx Item < 'tcx >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , .. }) = it . kind && let Some (trait_did) = of_trait . trait_ref . trait_def_id () && cx . tcx . is_diagnostic_item (sym :: ToString , trait_did) { span_lint_and_help (cx , TO_STRING_TRAIT_IMPL , it . span , "direct implementation of `ToString`" , None , "prefer implementing `Display` instead" ,) ; } } }
};
}

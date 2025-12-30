// Generated macro for impl_8130 (impl)
macro_rules! Depcrate_non_canonical_implsimpl_8130 {
() => {
// Module: crate::non_canonical_impls
// Provides: {"impl_8130"}
// Dependencies: {}
impl NonCanonicalImpls { pub (crate) fn new (tcx : TyCtxt < '_ >) -> Self { let lang_items = tcx . lang_items () ; Self { partial_ord_trait : lang_items . partial_ord_trait () , ord_trait : tcx . get_diagnostic_item (sym :: Ord) , clone_trait : lang_items . clone_trait () , copy_trait : lang_items . copy_trait () , } } }
};
}

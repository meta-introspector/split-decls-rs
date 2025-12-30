// Generated macro for is_deref_or_derefmut_impl (function)
macro_rules! Depcrate_dereferenceis_deref_or_derefmut_impl {
() => {
// Module: crate::dereference
// Provides: {"is_deref_or_derefmut_impl"}
// Dependencies: {}
fn is_deref_or_derefmut_impl (cx : & LateContext < '_ > , item : & Item < '_ >) -> bool { if let hir :: ItemKind :: Impl (impl_) = item . kind && let Some (of_trait) = impl_ . of_trait && let Some (trait_id) = of_trait . trait_ref . trait_def_id () { cx . tcx . lang_items () . deref_trait () == Some (trait_id) || cx . tcx . lang_items () . deref_mut_trait () == Some (trait_id) } else { false } }
};
}

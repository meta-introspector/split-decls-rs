// Generated macro for is_debug_impl (function)
macro_rules! Depcrate_writeis_debug_impl {
() => {
// Module: crate::write
// Provides: {"is_debug_impl"}
// Dependencies: {}
fn is_debug_impl (cx : & LateContext < '_ > , item : & Item < '_ >) -> bool { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , .. }) = & item . kind && let Some (trait_id) = of_trait . trait_ref . trait_def_id () { cx . tcx . is_diagnostic_item (sym :: Debug , trait_id) } else { false } }
};
}

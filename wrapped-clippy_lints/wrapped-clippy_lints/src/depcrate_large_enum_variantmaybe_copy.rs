// Generated macro for maybe_copy (function)
macro_rules! Depcrate_large_enum_variantmaybe_copy {
() => {
// Module: crate::large_enum_variant
// Provides: {"maybe_copy"}
// Dependencies: {}
fn maybe_copy < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { if let ty :: Adt (_def , args) = ty . kind () && args . types () . next () . is_some () && let Some (copy_trait) = cx . tcx . lang_items () . copy_trait () { return cx . tcx . non_blanket_impls_for_ty (copy_trait , ty) . next () . is_some () ; } false }
};
}

// Generated macro for is_hidden (function)
macro_rules! Depcrate_matches_match_wild_enumis_hidden {
() => {
// Module: crate::matches::match_wild_enum
// Provides: {"is_hidden"}
// Dependencies: {}
fn is_hidden (cx : & LateContext < '_ > , variant_def : & VariantDef) -> bool { cx . tcx . is_doc_hidden (variant_def . def_id) || cx . tcx . has_attr (variant_def . def_id , sym :: unstable) }
};
}

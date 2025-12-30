// Generated macro for is_public_macro (function)
macro_rules! Depcrate_macro_metavars_in_unsafeis_public_macro {
() => {
// Module: crate::macro_metavars_in_unsafe
// Provides: {"is_public_macro"}
// Dependencies: {}
fn is_public_macro (cx : & LateContext < '_ > , def_id : LocalDefId) -> bool { (cx . effective_visibilities . is_exported (def_id) || find_attr ! (cx . tcx . get_all_attrs (def_id) , AttributeKind :: MacroExport { .. })) && ! cx . tcx . is_doc_hidden (def_id) }
};
}

// Generated macro for type_alias_bounds (function)
macro_rules! Depcrate_lowertype_alias_bounds {
() => {
// Module: crate::lower
// Provides: {"type_alias_bounds"}
// Dependencies: {}
# [inline] pub (crate) fn type_alias_bounds < 'db > (db : & 'db dyn HirDatabase , type_alias : TypeAliasId ,) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { type_alias_bounds_with_diagnostics (db , type_alias) . 0 . as_ref () . map_bound (| it | & * * it) }
};
}

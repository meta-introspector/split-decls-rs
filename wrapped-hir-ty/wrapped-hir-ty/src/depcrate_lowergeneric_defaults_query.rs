// Generated macro for generic_defaults_query (function)
macro_rules! Depcrate_lowergeneric_defaults_query {
() => {
// Module: crate::lower
// Provides: {"generic_defaults_query"}
// Dependencies: {}
pub (crate) fn generic_defaults_query (db : & dyn HirDatabase , def : GenericDefId ,) -> GenericDefaults < '_ > { db . generic_defaults_with_diagnostics (def) . 0 }
};
}

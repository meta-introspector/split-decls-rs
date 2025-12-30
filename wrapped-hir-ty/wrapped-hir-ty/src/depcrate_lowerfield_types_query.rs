// Generated macro for field_types_query (function)
macro_rules! Depcrate_lowerfield_types_query {
() => {
// Module: crate::lower
// Provides: {"field_types_query"}
// Dependencies: {}
pub (crate) fn field_types_query < 'db > (db : & 'db dyn HirDatabase , variant_id : VariantId ,) -> Arc < ArenaMap < LocalFieldId , EarlyBinder < 'db , Ty < 'db > > > > { db . field_types_with_diagnostics (variant_id) . 0 }
};
}

// Generated macro for lt_from_placeholder_idx (function)
macro_rules! Depcrate_mappinglt_from_placeholder_idx {
() => {
// Module: crate::mapping
// Provides: {"lt_from_placeholder_idx"}
// Dependencies: {}
pub fn lt_from_placeholder_idx (db : & dyn HirDatabase , idx : PlaceholderIndex) -> LifetimeParamId { assert_eq ! (idx . ui , chalk_ir :: UniverseIndex :: ROOT) ; let interned_id = InternedLifetimeParamId :: from_id (unsafe { Id :: from_index (idx . idx . try_into () . unwrap ()) }) ; interned_id . loc (db) }
};
}

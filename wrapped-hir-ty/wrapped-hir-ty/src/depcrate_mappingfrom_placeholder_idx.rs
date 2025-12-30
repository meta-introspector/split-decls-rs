// Generated macro for from_placeholder_idx (function)
macro_rules! Depcrate_mappingfrom_placeholder_idx {
() => {
// Module: crate::mapping
// Provides: {"from_placeholder_idx"}
// Dependencies: {}
pub fn from_placeholder_idx (db : & dyn HirDatabase , idx : PlaceholderIndex) -> TypeOrConstParamId { assert_eq ! (idx . ui , chalk_ir :: UniverseIndex :: ROOT) ; let interned_id = InternedTypeOrConstParamId :: from_id (unsafe { Id :: from_index (idx . idx . try_into () . unwrap ()) }) ; interned_id . loc (db) }
};
}

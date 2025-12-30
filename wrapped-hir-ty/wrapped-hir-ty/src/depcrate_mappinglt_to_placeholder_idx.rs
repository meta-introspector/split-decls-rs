// Generated macro for lt_to_placeholder_idx (function)
macro_rules! Depcrate_mappinglt_to_placeholder_idx {
() => {
// Module: crate::mapping
// Provides: {"lt_to_placeholder_idx"}
// Dependencies: {}
pub fn lt_to_placeholder_idx (db : & dyn HirDatabase , id : LifetimeParamId) -> PlaceholderIndex { let interned_id = InternedLifetimeParamId :: new (db , id) ; PlaceholderIndex { ui : chalk_ir :: UniverseIndex :: ROOT , idx : interned_id . as_id () . index () as usize , } }
};
}

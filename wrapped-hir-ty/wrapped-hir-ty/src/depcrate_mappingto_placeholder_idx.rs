// Generated macro for to_placeholder_idx (function)
macro_rules! Depcrate_mappingto_placeholder_idx {
() => {
// Module: crate::mapping
// Provides: {"to_placeholder_idx"}
// Dependencies: {}
pub fn to_placeholder_idx (db : & dyn HirDatabase , id : TypeOrConstParamId) -> PlaceholderIndex { let interned_id = InternedTypeOrConstParamId :: new (db , id) ; PlaceholderIndex { ui : chalk_ir :: UniverseIndex :: ROOT , idx : interned_id . as_id () . index () as usize , } }
};
}

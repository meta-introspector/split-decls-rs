// Generated macro for feature_gate_to_pointer_width (function)
macro_rules! Depcrate_generic_const_mappingsfeature_gate_to_pointer_width {
() => {
// Module: crate::generic_const_mappings
// Provides: {"feature_gate_to_pointer_width"}
// Dependencies: {}
const fn feature_gate_to_pointer_width (uint : u64) -> & 'static str { if uint > u32 :: MAX as u64 { r#"#[cfg(target_pointer_width = "64")]"# } else if uint > u16 :: MAX as u64 { r#"#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]"# } else { "" } }
};
}

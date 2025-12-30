// Generated macro for is_system_type (function)
macro_rules! Depcrate_registryis_system_type {
() => {
// Module: crate::registry
// Provides: {"is_system_type"}
// Dependencies: {}
fn is_system_type (name : & str) -> bool { if name . starts_with ("__") { return true ; } name == "Boolean" || name == "Int" || name == "Float" || name == "String" || name == "ID" }
};
}

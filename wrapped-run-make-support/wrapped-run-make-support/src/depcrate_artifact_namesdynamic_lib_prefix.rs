// Generated macro for dynamic_lib_prefix (function)
macro_rules! Depcrate_artifact_namesdynamic_lib_prefix {
() => {
// Module: crate::artifact_names
// Provides: {"dynamic_lib_prefix"}
// Dependencies: {}
fn dynamic_lib_prefix () -> & 'static str { if target () . contains ("windows") { "" } else { "lib" } }
};
}

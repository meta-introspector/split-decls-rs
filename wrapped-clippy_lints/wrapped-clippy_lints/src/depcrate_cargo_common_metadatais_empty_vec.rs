// Generated macro for is_empty_vec (function)
macro_rules! Depcrate_cargo_common_metadatais_empty_vec {
() => {
// Module: crate::cargo::common_metadata
// Provides: {"is_empty_vec"}
// Dependencies: {}
fn is_empty_vec (value : & [String]) -> bool { value . iter () . all (String :: is_empty) }
};
}

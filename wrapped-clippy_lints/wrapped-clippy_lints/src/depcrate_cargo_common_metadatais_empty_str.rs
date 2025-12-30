// Generated macro for is_empty_str (function)
macro_rules! Depcrate_cargo_common_metadatais_empty_str {
() => {
// Module: crate::cargo::common_metadata
// Provides: {"is_empty_str"}
// Dependencies: {}
fn is_empty_str < T : AsRef < std :: ffi :: OsStr > > (value : Option < & T >) -> bool { value . is_none_or (| s | s . as_ref () . is_empty ()) }
};
}

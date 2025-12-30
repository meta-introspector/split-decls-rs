// Generated macro for missing_warning (function)
macro_rules! Depcrate_cargo_common_metadatamissing_warning {
() => {
// Module: crate::cargo::common_metadata
// Provides: {"missing_warning"}
// Dependencies: {}
fn missing_warning (cx : & LateContext < '_ > , package : & cargo_metadata :: Package , field : & str) { let message = format ! ("package `{}` is missing `{field}` metadata" , package . name) ; span_lint (cx , CARGO_COMMON_METADATA , DUMMY_SP , message) ; }
};
}

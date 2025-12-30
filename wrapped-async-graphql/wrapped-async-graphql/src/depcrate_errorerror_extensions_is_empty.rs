// Generated macro for error_extensions_is_empty (function)
macro_rules! Depcrate_errorerror_extensions_is_empty {
() => {
// Module: crate::error
// Provides: {"error_extensions_is_empty"}
// Dependencies: {}
fn error_extensions_is_empty (values : & Option < ErrorExtensionValues >) -> bool { values . as_ref () . is_none_or (| values | values . 0 . is_empty ()) }
};
}

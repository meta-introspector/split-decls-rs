// Generated macro for is_invalid_ident (function)
macro_rules! Depcrate_redundant_test_prefixis_invalid_ident {
() => {
// Module: crate::redundant_test_prefix
// Provides: {"is_invalid_ident"}
// Dependencies: {}
fn is_invalid_ident (ident : Symbol) -> bool { ident . is_reserved (| | edition :: LATEST_STABLE_EDITION) || ! is_ident (ident . as_str ()) }
};
}

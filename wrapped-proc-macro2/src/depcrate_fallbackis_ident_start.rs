// Generated macro for is_ident_start (function)
macro_rules! Depcrate_fallbackis_ident_start {
() => {
// Module: crate::fallback
// Provides: {"is_ident_start"}
// Dependencies: {}
pub (crate) fn is_ident_start (c : char) -> bool { c == '_' || unicode_ident :: is_xid_start (c) }
};
}

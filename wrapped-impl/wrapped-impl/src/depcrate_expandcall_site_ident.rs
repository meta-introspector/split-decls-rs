// Generated macro for call_site_ident (function)
macro_rules! Depcrate_expandcall_site_ident {
() => {
// Module: crate::expand
// Provides: {"call_site_ident"}
// Dependencies: {}
pub (crate) fn call_site_ident (ident : & Ident) -> Ident { let mut ident = ident . clone () ; ident . set_span (ident . span () . resolved_at (Span :: call_site ())) ; ident }
};
}

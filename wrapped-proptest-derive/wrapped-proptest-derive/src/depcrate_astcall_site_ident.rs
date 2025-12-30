// Generated macro for call_site_ident (function)
macro_rules! Depcrate_astcall_site_ident {
() => {
// Module: crate::ast
// Provides: {"call_site_ident"}
// Dependencies: {}
fn call_site_ident (ident : & str) -> syn :: Ident { syn :: Ident :: new (ident , Span :: call_site ()) }
};
}

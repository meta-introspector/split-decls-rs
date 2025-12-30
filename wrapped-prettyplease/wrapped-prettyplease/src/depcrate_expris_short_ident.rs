// Generated macro for is_short_ident (function)
macro_rules! Depcrate_expris_short_ident {
() => {
// Module: crate::expr
// Provides: {"is_short_ident"}
// Dependencies: {}
fn is_short_ident (expr : & Expr) -> bool { if let Expr :: Path (expr) = expr { return expr . attrs . is_empty () && expr . qself . is_none () && expr . path . get_ident () . map_or (false , | ident | ident . to_string () . len () as isize <= INDENT) ; } false }
};
}

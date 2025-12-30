// Generated macro for respan (function)
macro_rules! Depcraterespan {
() => {
// Module: crate
// Provides: {"respan"}
// Dependencies: {}
fn respan (ts : TokenStream , span : Span) -> TokenStream { let mut res = TokenStream :: new () ; for tt in ts { let tt = match tt { TokenTree :: Ident (mut ident) => { ident . set_span (ident . span () . resolved_at (span) . located_at (span)) ; TokenTree :: Ident (ident) } TokenTree :: Group (group) => { TokenTree :: Group (Group :: new (group . delimiter () , respan (group . stream () , span))) } _ => tt , } ; res . extend (Some (tt)) } res }
};
}

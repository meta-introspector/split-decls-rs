// Generated macro for mk_ident (function)
macro_rules! Depcratemk_ident {
() => {
// Module: crate
// Provides: {"mk_ident"}
// Dependencies: {}
fn mk_ident (i : usize) -> Ident { Ident :: new (& format ! ("__{}" , i) , Span :: call_site ()) }
};
}

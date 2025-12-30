// Generated macro for impl_11 (impl)
macro_rules! Depcrate_automockimpl_11 {
() => {
// Module: crate::automock
// Provides: {"impl_11"}
// Dependencies: {}
impl Parse for Attr { fn parse (input : ParseStream) -> parse :: Result < Self > { let lookahead = input . lookahead1 () ; if lookahead . peek (Token ! [type]) { input . parse () . map (Attr :: Type) } else if lookahead . peek (Ident) { let ident : Ident = input . parse () ? ; if ident == "target" { let _eq : Token ! [=] = input . parse () ? ; let target : Ident = input . parse () ? ; Ok (Attr :: Target (target)) } else { Err (Error :: new (ident . span () , "expected 'target'")) } } else { Err (lookahead . error ()) } } }
};
}

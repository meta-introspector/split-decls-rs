// Generated macro for impl_285 (impl)
macro_rules! Depcrate_parseimpl_285 {
() => {
// Module: crate::parse
// Provides: {"impl_285"}
// Dependencies: {}
impl Parse for Attribute { fn parse (input : ParseStream) -> syn :: Result < Self > { if input . peek2 (Token ! [<]) { let tag = input . parse () ? ; let _open = input . parse :: < Token ! [<] > () ? ; let inner = input . parse () ? ; let _close = input . parse :: < Token ! [>] > () ? ; Ok (Attribute :: Type (tag , inner)) } else if input . peek2 (Token ! [::]) { let inner = input . parse () ? ; Ok (Attribute :: Attr (inner)) } else if input . peek2 (token :: Paren) { let tag = input . parse () ? ; let content ; let _ = syn :: parenthesized ! (content in input) ; let args = Punctuated :: < Ident , Token ! [,] > :: parse_terminated (& content) ? . into_iter () . map (IntoPat :: into_pat) . collect () ; Ok (Attribute :: Tagged (tag , args)) } else { Ok (Attribute :: Attr (input . parse () ?)) } } }
};
}

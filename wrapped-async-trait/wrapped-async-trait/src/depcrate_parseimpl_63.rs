// Generated macro for impl_63 (impl)
macro_rules! Depcrate_parseimpl_63 {
() => {
// Module: crate::parse
// Provides: {"impl_63"}
// Dependencies: {}
impl Parse for Item { fn parse (input : ParseStream) -> Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; let mut lookahead = input . lookahead1 () ; if lookahead . peek (Token ! [unsafe]) { let ahead = input . fork () ; ahead . parse :: < Token ! [unsafe] > () ? ; lookahead = ahead . lookahead1 () ; } if lookahead . peek (Token ! [pub]) || lookahead . peek (Token ! [trait]) { let mut item : ItemTrait = input . parse () ? ; item . attrs = attrs ; Ok (Item :: Trait (item)) } else if lookahead . peek (Token ! [impl]) { let mut item : ItemImpl = input . parse () ? ; if item . trait_ . is_none () { return Err (Error :: new (Span :: call_site () , "expected a trait impl")) ; } item . attrs = attrs ; Ok (Item :: Impl (item)) } else { Err (lookahead . error ()) } } }
};
}

// Generated macro for parse_oibits (function)
macro_rules! Depcrate_parseparse_oibits {
() => {
// Module: crate::parse
// Provides: {"parse_oibits"}
// Dependencies: {}
fn parse_oibits (input : ParseStream) -> parse :: Result < Punctuated < Ident , Token ! [+] > > { let mut out = Punctuated :: new () ; if input . parse :: < Option < Token ! [:] > > () ? . is_some () { loop { out . push_value (input . call (parse_oibit) ?) ; if input . peek (token :: Brace) { break ; } out . push_punct (input . parse () ?) ; if input . peek (token :: Brace) { break ; } } } Ok (out) }
};
}

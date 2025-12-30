// Generated macro for parse_oibit (function)
macro_rules! Depcrate_parseparse_oibit {
() => {
// Module: crate::parse
// Provides: {"parse_oibit"}
// Dependencies: {}
fn parse_oibit (input : ParseStream) -> parse :: Result < Ident > { let lookahead = input . lookahead1 () ; if lookahead . peek (kw :: Sync) || lookahead . peek (kw :: Send) { input . parse () } else { Err (lookahead . error ()) } }
};
}

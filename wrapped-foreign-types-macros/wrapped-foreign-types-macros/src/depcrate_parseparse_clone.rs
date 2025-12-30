// Generated macro for parse_clone (function)
macro_rules! Depcrate_parseparse_clone {
() => {
// Module: crate::parse
// Provides: {"parse_clone"}
// Dependencies: {}
fn parse_clone (input : ParseStream) -> parse :: Result < Option < Expr > > { if input . peek (Token ! [fn]) && input . peek2 (kw :: clone) { input . call (parse_fn :: < kw :: clone >) . map (Some) } else { Ok (None) } }
};
}

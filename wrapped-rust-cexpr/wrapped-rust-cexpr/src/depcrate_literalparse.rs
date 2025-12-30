// Generated macro for parse (function)
macro_rules! Depcrate_literalparse {
() => {
// Module: crate::literal
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse a C literal."] # [doc = ""] # [doc = " The input must contain exactly the representation of a single literal"] # [doc = " token, and in particular no whitespace or sign prefixes."] pub fn parse (input : & [u8]) -> IResult < & [u8] , EvalResult , crate :: Error < & [u8] > > { crate :: assert_full_parse (one_literal (input)) }
};
}

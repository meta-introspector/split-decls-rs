// Generated macro for parse_ident_or_array (function)
macro_rules! Depcrate_parseparse_ident_or_array {
() => {
// Module: crate::parse
// Provides: {"parse_ident_or_array"}
// Dependencies: {}
# [doc = " Parse either a single identifier (`foo`) or an array of identifiers (`[foo, bar, baz]`)."] fn parse_ident_or_array (input : ParseStream) -> syn :: Result < Vec < Ident > > { if ! input . peek (token :: Bracket) { return Ok (vec ! [input . parse () ?]) ; } parse_ident_array (input) }
};
}

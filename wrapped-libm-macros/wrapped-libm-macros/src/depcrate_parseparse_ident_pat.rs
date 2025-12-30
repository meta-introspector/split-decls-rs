// Generated macro for parse_ident_pat (function)
macro_rules! Depcrate_parseparse_ident_pat {
() => {
// Module: crate::parse
// Provides: {"parse_ident_pat"}
// Dependencies: {}
# [doc = " Parse an pattern of idents, specifically `(foo | bar | baz)`."] fn parse_ident_pat (input : ParseStream) -> syn :: Result < Vec < Ident > > { if ! input . peek2 (Token ! [|]) { return Ok (vec ! [input . parse () ?]) ; } let fields = Punctuated :: < Ident , Token ! [|] > :: parse_separated_nonempty (input) ? ; Ok (fields . into_iter () . collect ()) }
};
}

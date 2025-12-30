// Generated macro for parse_ident_array (function)
macro_rules! Depcrate_parseparse_ident_array {
() => {
// Module: crate::parse
// Provides: {"parse_ident_array"}
// Dependencies: {}
# [doc = " Parse an array of idents, e.g. `[foo, bar, baz]`."] fn parse_ident_array (input : ParseStream) -> syn :: Result < Vec < Ident > > { let content ; let _ = bracketed ! (content in input) ; let fields = content . parse_terminated (Ident :: parse , Token ! [,]) ? ; Ok (fields . into_iter () . collect ()) }
};
}

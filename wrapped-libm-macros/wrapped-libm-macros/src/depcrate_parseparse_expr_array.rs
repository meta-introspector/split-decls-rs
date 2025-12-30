// Generated macro for parse_expr_array (function)
macro_rules! Depcrate_parseparse_expr_array {
() => {
// Module: crate::parse
// Provides: {"parse_expr_array"}
// Dependencies: {}
# [doc = " Parse an array of expressions."] fn parse_expr_array (input : ParseStream) -> syn :: Result < Vec < Expr > > { let content ; let _ = bracketed ! (content in input) ; let fields = content . parse_terminated (Expr :: parse , Token ! [,]) ? ; Ok (fields . into_iter () . collect ()) }
};
}

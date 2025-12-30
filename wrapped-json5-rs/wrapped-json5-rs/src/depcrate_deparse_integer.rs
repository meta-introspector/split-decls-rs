// Generated macro for parse_integer (function)
macro_rules! Depcrate_deparse_integer {
() => {
// Module: crate::de
// Provides: {"parse_integer"}
// Dependencies: {}
fn parse_integer (pair : & Pair < '_ , Rule >) -> Result < i64 > { match pair . as_str () { s if is_hex_literal (s) => Ok (parse_hex (& s [2 ..]) ? as i64) , s => s . parse () . or_else (| _ | Err (de :: Error :: custom ("error parsing integer"))) , } }
};
}

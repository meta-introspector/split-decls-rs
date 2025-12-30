// Generated macro for parse_number (function)
macro_rules! Depcrate_deparse_number {
() => {
// Module: crate::de
// Provides: {"parse_number"}
// Dependencies: {}
fn parse_number (pair : & Pair < '_ , Rule >) -> Result < f64 > { match pair . as_str () { "Infinity" => Ok (f64 :: INFINITY) , "-Infinity" => Ok (f64 :: NEG_INFINITY) , "NaN" | "-NaN" => Ok (f64 :: NAN) , s if is_hex_literal (s) => parse_hex (& s [2 ..]) . map (f64 :: from) , s => { if let Ok (r) = s . parse :: < f64 > () { if r . is_finite () { Ok (r) } else { Err (de :: Error :: custom ("error parsing number: too large")) } } else { Err (de :: Error :: custom ("error parsing number")) } } } }
};
}

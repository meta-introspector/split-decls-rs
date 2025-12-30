// Generated macro for check (macro)
macro_rules! Depcrate_float_testscheck {
() => {
// Module: crate::float::tests
// Provides: {"check"}
// Dependencies: {}
# [doc = " Helper macro to check parsing a float."] # [doc = ""] # [doc = " This macro contains quite a bit of logic itself (which can be buggy of"] # [doc = " course), so we have a few test functions below to test a bunch of cases"] # [doc = " manually."] macro_rules ! check { ($ intpart : literal $ fracpart : literal $ exppart : literal $ suffix : tt) => { let input = concat ! ($ intpart , $ fracpart , $ exppart , check ! (@ stringify_suffix $ suffix)) ; let expected_float = FloatLit { raw : input , end_integer_part : $ intpart . len () , end_fractional_part : $ intpart . len () + $ fracpart . len () , end_number_part : $ intpart . len () + $ fracpart . len () + $ exppart . len () , } ; assert_parse_ok_eq (input , FloatLit :: parse (input) , expected_float . clone () , "FloatLit::parse") ; assert_parse_ok_eq (input , Literal :: parse (input) , Literal :: Float (expected_float) , "Literal::parse") ; assert_eq ! (FloatLit :: parse (input) . unwrap () . suffix () , check ! (@ ty $ suffix)) ; assert_roundtrip (expected_float . to_owned () , input) ; } ; (@ ty f32) => { "f32" } ; (@ ty f64) => { "f64" } ; (@ ty -) => { "" } ; (@ stringify_suffix -) => { "" } ; (@ stringify_suffix $ suffix : ident) => { stringify ! ($ suffix) } ; }
};
}

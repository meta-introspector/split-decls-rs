// Generated macro for check (macro)
macro_rules! Depcrate_cstr_testscheck {
() => {
// Module: crate::cstr::tests
// Provides: {"check"}
// Dependencies: {}
macro_rules ! check { ($ lit : literal , $ has_escapes : expr , $ num_hashes : expr) => { check ! ($ lit , stringify ! ($ lit) , $ has_escapes , $ num_hashes , "") } ; ($ lit : literal , $ input : expr , $ has_escapes : expr , $ num_hashes : expr , $ suffix : literal) => { let input = $ input ; let expected = CStringLit { raw : input , value : $ lit . to_owned () , num_hashes : $ num_hashes , start_suffix : input . len () - $ suffix . len () , } ; assert_parse_ok_eq (input , CStringLit :: parse (input) , expected . clone () , "CStringLit::parse") ; assert_parse_ok_eq (input , Literal :: parse (input) , Literal :: CString (expected . clone ()) , "Literal::parse") ; let lit = CStringLit :: parse (input) . unwrap () ; assert_eq ! (lit . value () , $ lit) ; assert_eq ! (lit . suffix () , $ suffix) ; assert_eq ! (lit . into_value () . as_ref () , $ lit) ; assert_roundtrip (expected . into_owned () , input) ; } ; }
};
}

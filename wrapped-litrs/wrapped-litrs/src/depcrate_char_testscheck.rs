// Generated macro for check (macro)
macro_rules! Depcrate_char_testscheck {
() => {
// Module: crate::char::tests
// Provides: {"check"}
// Dependencies: {}
macro_rules ! check { ($ lit : literal) => { check ! ($ lit , stringify ! ($ lit) , "") } ; ($ lit : literal , $ input : expr , $ suffix : literal) => { let input = $ input ; let expected = CharLit { raw : input , start_suffix : input . len () - $ suffix . len () , value : $ lit , } ; assert_parse_ok_eq (input , CharLit :: parse (input) , expected . clone () , "CharLit::parse") ; assert_parse_ok_eq (input , Literal :: parse (input) , Literal :: Char (expected) , "Literal::parse") ; let lit = CharLit :: parse (input) . unwrap () ; assert_eq ! (lit . value () , $ lit) ; assert_eq ! (lit . suffix () , $ suffix) ; assert_roundtrip (expected . to_owned () , input) ; } ; }
};
}

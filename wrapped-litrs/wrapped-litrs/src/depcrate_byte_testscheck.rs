// Generated macro for check (macro)
macro_rules! Depcrate_byte_testscheck {
() => {
// Module: crate::byte::tests
// Provides: {"check"}
// Dependencies: {}
macro_rules ! check { ($ lit : literal) => { check ! ($ lit , stringify ! ($ lit) , "") } ; ($ lit : literal , $ input : expr , $ suffix : literal) => { let input = $ input ; let expected = ByteLit { raw : input , start_suffix : input . len () - $ suffix . len () , value : $ lit , } ; assert_parse_ok_eq (input , ByteLit :: parse (input) , expected . clone () , "ByteLit::parse") ; assert_parse_ok_eq (input , Literal :: parse (input) , Literal :: Byte (expected) , "Literal::parse") ; let lit = ByteLit :: parse (input) . unwrap () ; assert_eq ! (lit . value () , $ lit) ; assert_eq ! (lit . suffix () , $ suffix) ; assert_roundtrip (expected . to_owned () , input) ; } ; }
};
}

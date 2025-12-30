// Generated macro for assert_bool_parse (macro)
macro_rules! Depcrate_bool_testsassert_bool_parse {
() => {
// Module: crate::bool::tests
// Provides: {"assert_bool_parse"}
// Dependencies: {}
macro_rules ! assert_bool_parse { ($ input : literal , $ expected : expr) => { assert_parse_ok_eq ($ input , Literal :: parse ($ input) , Literal :: Bool ($ expected) , "Literal::parse" ,) ; assert_parse_ok_eq ($ input , BoolLit :: parse ($ input) , $ expected , "BoolLit::parse") ; } ; }
};
}

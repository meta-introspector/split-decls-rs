// Generated macro for assert_parse (macro)
macro_rules! Depcrate_combinator_testsassert_parse {
() => {
// Module: crate::combinator::tests
// Provides: {"assert_parse"}
// Dependencies: {}
macro_rules ! assert_parse (($ left : expr , $ right : expr) => { let res : $ crate :: IResult < _ , _ , (_ , ErrorKind) > = $ left ; assert_eq ! (res , $ right) ; } ;) ;
};
}

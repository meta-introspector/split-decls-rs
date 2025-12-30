// Generated macro for parse_ok (function)
macro_rules! Depcrate_testsparse_ok {
() => {
// Module: crate::tests
// Provides: {"parse_ok"}
// Dependencies: {}
# [test] fn parse_ok () { for case in TestCase :: list ("parser/ok") { let _guard = stdx :: panic_context :: enter (format ! ("{:?}" , case . rs)) ; let (actual , errors) = parse (TopEntryPoint :: SourceFile , & case . text , Edition :: CURRENT) ; assert ! (! errors , "errors in an OK file {}:\n{actual}" , case . rs . display ()) ; expect_file ! [case . rast] . assert_eq (& actual) ; } }
};
}

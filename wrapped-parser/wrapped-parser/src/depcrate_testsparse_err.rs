// Generated macro for parse_err (function)
macro_rules! Depcrate_testsparse_err {
() => {
// Module: crate::tests
// Provides: {"parse_err"}
// Dependencies: {}
# [test] fn parse_err () { for case in TestCase :: list ("parser/err") { let _guard = stdx :: panic_context :: enter (format ! ("{:?}" , case . rs)) ; let (actual , errors) = parse (TopEntryPoint :: SourceFile , & case . text , Edition :: CURRENT) ; assert ! (errors , "no errors in an ERR file {}:\n{actual}" , case . rs . display ()) ; expect_file ! [case . rast] . assert_eq (& actual) } }
};
}

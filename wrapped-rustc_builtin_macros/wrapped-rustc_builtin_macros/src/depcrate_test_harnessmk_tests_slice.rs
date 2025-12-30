// Generated macro for mk_tests_slice (function)
macro_rules! Depcrate_test_harnessmk_tests_slice {
() => {
// Module: crate::test_harness
// Provides: {"mk_tests_slice"}
// Dependencies: {}
# [doc = " Creates a slice containing every test like so:"] # [doc = " &[&test1, &test2]"] fn mk_tests_slice (cx : & TestCtxt < '_ > , sp : Span) -> Box < ast :: Expr > { debug ! ("building test vector from {} tests" , cx . test_cases . len ()) ; let ecx = & cx . ext_cx ; let mut tests = cx . test_cases . clone () ; tests . sort_by (| a , b | a . name . as_str () . cmp (b . name . as_str ())) ; ecx . expr_array_ref (sp , tests . iter () . map (| test | { ecx . expr_addr_of (test . span , ecx . expr_path (ecx . path (test . span , vec ! [test . ident]))) }) . collect () ,) }
};
}

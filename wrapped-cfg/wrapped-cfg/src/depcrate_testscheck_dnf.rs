// Generated macro for check_dnf (function)
macro_rules! Depcrate_testscheck_dnf {
() => {
// Module: crate::tests
// Provides: {"check_dnf"}
// Dependencies: {}
fn check_dnf (input : & str , expect : Expect) { let source_file = ast :: SourceFile :: parse (input , Edition :: CURRENT) . ok () . unwrap () ; let tt = source_file . syntax () . descendants () . find_map (ast :: TokenTree :: cast) . unwrap () ; let tt = syntax_node_to_token_tree (tt . syntax () , DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: ProcMacro ,) ; let cfg = CfgExpr :: parse (& tt) ; let actual = format ! ("#![cfg({})]" , DnfExpr :: new (& cfg)) ; expect . assert_eq (& actual) ; }
};
}

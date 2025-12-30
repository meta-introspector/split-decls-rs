// Generated macro for check_why_inactive (function)
macro_rules! Depcrate_testscheck_why_inactive {
() => {
// Module: crate::tests
// Provides: {"check_why_inactive"}
// Dependencies: {}
fn check_why_inactive (input : & str , opts : & CfgOptions , expect : Expect) { let source_file = ast :: SourceFile :: parse (input , Edition :: CURRENT) . ok () . unwrap () ; let tt = source_file . syntax () . descendants () . find_map (ast :: TokenTree :: cast) . unwrap () ; let tt = syntax_node_to_token_tree (tt . syntax () , DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: ProcMacro ,) ; let cfg = CfgExpr :: parse (& tt) ; let dnf = DnfExpr :: new (& cfg) ; let why_inactive = dnf . why_inactive (opts) . unwrap () . to_string () ; expect . assert_eq (& why_inactive) ; }
};
}

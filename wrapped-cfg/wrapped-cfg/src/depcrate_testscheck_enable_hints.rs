// Generated macro for check_enable_hints (function)
macro_rules! Depcrate_testscheck_enable_hints {
() => {
// Module: crate::tests
// Provides: {"check_enable_hints"}
// Dependencies: {}
# [track_caller] fn check_enable_hints (input : & str , opts : & CfgOptions , expected_hints : & [& str]) { let source_file = ast :: SourceFile :: parse (input , Edition :: CURRENT) . ok () . unwrap () ; let tt = source_file . syntax () . descendants () . find_map (ast :: TokenTree :: cast) . unwrap () ; let tt = syntax_node_to_token_tree (tt . syntax () , DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: ProcMacro ,) ; let cfg = CfgExpr :: parse (& tt) ; let dnf = DnfExpr :: new (& cfg) ; let hints = dnf . compute_enable_hints (opts) . map (| diff | diff . to_string ()) . collect :: < Vec < _ > > () ; assert_eq ! (hints , expected_hints) ; }
};
}

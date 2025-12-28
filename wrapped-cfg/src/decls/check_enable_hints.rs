macro_rules! deps {
    () => {
        CfgOptions!();
        CfgExpr!();
        DnfExpr!();
    };
}

macro_rules! check_enable_hints {
    () => {
        deps!();
        # [track_caller] fn check_enable_hints (input : & str , opts : & CfgOptions , expected_hints : & [& str]) { let source_file = ast :: SourceFile :: parse (input , Edition :: CURRENT) . ok () . unwrap () ; let tt = source_file . syntax () . descendants () . find_map (ast :: TokenTree :: cast) . unwrap () ; let tt = syntax_node_to_token_tree (tt . syntax () , DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: ProcMacro ,) ; let cfg = CfgExpr :: parse (& tt) ; let dnf = DnfExpr :: new (& cfg) ; let hints = dnf . compute_enable_hints (opts) . map (| diff | diff . to_string ()) . collect :: < Vec < _ > > () ; assert_eq ! (hints , expected_hints) ; }
    };
}

check_enable_hints!()
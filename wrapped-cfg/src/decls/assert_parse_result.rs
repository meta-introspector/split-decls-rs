macro_rules! deps {
    () => {
        CfgExpr!();
    };
}

macro_rules! assert_parse_result {
    () => {
        deps!();
        fn assert_parse_result (input : & str , expected : CfgExpr) { let source_file = ast :: SourceFile :: parse (input , Edition :: CURRENT) . ok () . unwrap () ; let tt = source_file . syntax () . descendants () . find_map (ast :: TokenTree :: cast) . unwrap () ; let tt = syntax_node_to_token_tree (tt . syntax () , DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: ProcMacro ,) ; let cfg = CfgExpr :: parse (& tt) ; assert_eq ! (cfg , expected) ; }
    };
}

assert_parse_result!()
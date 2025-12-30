// Generated macro for tests (module)
macro_rules! Depcrate_target_spectests {
() => {
// Module: crate::target_spec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use ide :: Edition ; use syntax :: { SmolStr , ast :: { self , AstNode } , } ; use syntax_bridge :: { DocCommentDesugarMode , dummy_test_span_utils :: { DUMMY , DummyTestSpanMap } , syntax_node_to_token_tree , } ; fn check (cfg : & str , expected_features : & [& str]) { let cfg_expr = { let source_file = ast :: SourceFile :: parse (cfg , Edition :: CURRENT) . ok () . unwrap () ; let tt = source_file . syntax () . descendants () . find_map (ast :: TokenTree :: cast) . unwrap () ; let tt = syntax_node_to_token_tree (tt . syntax () , & DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: Mbe ,) ; CfgExpr :: parse (& tt) } ; let mut features = vec ! [] ; required_features (& cfg_expr , & mut features) ; let expected_features = expected_features . iter () . map (| & it | SmolStr :: new (it)) . collect :: < Vec < _ > > () ; assert_eq ! (features , expected_features) ; } # [test] fn test_cfg_expr_minimal_features_needed () { check (r#"#![cfg(feature = "baz")]"# , & ["baz"]) ; check (r#"#![cfg(all(feature = "baz", feature = "foo"))]"# , & ["baz" , "foo"]) ; check (r#"#![cfg(any(feature = "baz", feature = "foo", unix))]"# , & ["baz"]) ; check (r#"#![cfg(foo)]"# , & []) ; } }
};
}

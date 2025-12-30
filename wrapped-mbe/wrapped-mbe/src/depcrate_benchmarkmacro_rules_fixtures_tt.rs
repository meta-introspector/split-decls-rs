// Generated macro for macro_rules_fixtures_tt (function)
macro_rules! Depcrate_benchmarkmacro_rules_fixtures_tt {
() => {
// Module: crate::benchmark
// Provides: {"macro_rules_fixtures_tt"}
// Dependencies: {}
fn macro_rules_fixtures_tt () -> FxHashMap < String , tt :: TopSubtree < Span > > { let fixture = bench_fixture :: numerous_macro_rules () ; let source_file = ast :: SourceFile :: parse (& fixture , span :: Edition :: CURRENT) . ok () . unwrap () ; source_file . syntax () . descendants () . filter_map (ast :: MacroRules :: cast) . map (| rule | { let id = rule . name () . unwrap () . to_string () ; let def_tt = syntax_node_to_token_tree (rule . token_tree () . unwrap () . syntax () , DummyTestSpanMap , DUMMY , DocCommentDesugarMode :: Mbe ,) ; (id , def_tt) }) . collect () }
};
}

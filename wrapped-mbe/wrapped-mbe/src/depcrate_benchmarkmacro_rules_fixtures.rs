// Generated macro for macro_rules_fixtures (function)
macro_rules! Depcrate_benchmarkmacro_rules_fixtures {
() => {
// Module: crate::benchmark
// Provides: {"macro_rules_fixtures"}
// Dependencies: {}
fn macro_rules_fixtures () -> FxHashMap < String , DeclarativeMacro > { macro_rules_fixtures_tt () . into_iter () . sorted_by_key (| (id , _) | id . clone ()) . map (| (id , tt) | (id , DeclarativeMacro :: parse_macro_rules (& tt , | _ | span :: Edition :: CURRENT))) . collect () }
};
}

// Generated macro for filter_edges (function)
macro_rules! Depcrate_assert_dep_graphfilter_edges {
() => {
// Module: crate::assert_dep_graph
// Provides: {"filter_edges"}
// Dependencies: {}
fn filter_edges (query : & DepGraphQuery , nodes : & FxIndexSet < DepKind >) -> Vec < (DepKind , DepKind) > { let uniq : FxIndexSet < _ > = query . edges () . into_iter () . map (| (s , t) | (s . kind , t . kind)) . filter (| (source , target) | nodes . contains (source) && nodes . contains (target)) . collect () ; uniq . into_iter () . collect () }
};
}

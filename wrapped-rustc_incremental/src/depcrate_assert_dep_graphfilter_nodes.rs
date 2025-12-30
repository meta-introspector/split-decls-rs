// Generated macro for filter_nodes (function)
macro_rules! Depcrate_assert_dep_graphfilter_nodes {
() => {
// Module: crate::assert_dep_graph
// Provides: {"filter_nodes"}
// Dependencies: {}
fn filter_nodes < 'q > (query : & 'q DepGraphQuery , sources : & Option < FxIndexSet < & 'q DepNode > > , targets : & Option < FxIndexSet < & 'q DepNode > > ,) -> FxIndexSet < DepKind > { if let Some (sources) = sources { if let Some (targets) = targets { walk_between (query , sources , targets) } else { walk_nodes (query , sources , OUTGOING) } } else if let Some (targets) = targets { walk_nodes (query , targets , INCOMING) } else { query . nodes () . into_iter () . map (| n | n . kind) . collect () } }
};
}

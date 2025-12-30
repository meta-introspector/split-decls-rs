// Generated macro for node_set (function)
macro_rules! Depcrate_assert_dep_graphnode_set {
() => {
// Module: crate::assert_dep_graph
// Provides: {"node_set"}
// Dependencies: {}
fn node_set < 'q > (query : & 'q DepGraphQuery , filter : & DepNodeFilter ,) -> Option < FxIndexSet < & 'q DepNode > > { debug ! ("node_set(filter={:?})" , filter) ; if filter . accepts_all () { return None ; } Some (query . nodes () . into_iter () . filter (| n | filter . test (n)) . collect ()) }
};
}

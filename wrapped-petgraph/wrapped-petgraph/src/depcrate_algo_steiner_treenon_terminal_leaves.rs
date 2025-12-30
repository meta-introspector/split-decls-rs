// Generated macro for non_terminal_leaves (function)
macro_rules! Depcrate_algo_steiner_treenon_terminal_leaves {
() => {
// Module: crate::algo::steiner_tree
// Provides: {"non_terminal_leaves"}
// Dependencies: {}
fn non_terminal_leaves < G > (graph : G , terminals : & [G :: NodeId]) -> HashSet < G :: NodeId > where G : GraphBase + IntoNodeReferences + IntoNodeIdentifiers + IntoNeighbors , G :: NodeId : Hash + Eq + Debug , G :: NodeRef : Eq + Hash , { let mut removed_leaves = HashSet :: new () ; let mut remaining_leaves = graph . node_identifiers () . filter (| node_id | { graph . neighbors (* node_id) . collect :: < HashSet < _ > > () . len () == 1 && ! terminals . contains (node_id) }) . collect :: < HashSet < _ > > () ; while ! remaining_leaves . is_empty () { remaining_leaves = graph . node_identifiers () . filter (| node_id | { ! terminals . contains (node_id) && ! removed_leaves . contains (node_id) && (graph . neighbors (* node_id) . collect :: < HashSet < _ > > () . difference (& removed_leaves)) . collect :: < Vec < _ > > () . len () == 1 }) . collect :: < HashSet < _ > > () ; removed_leaves = removed_leaves . union (& remaining_leaves) . cloned () . collect :: < HashSet < _ > > () ; } removed_leaves }
};
}

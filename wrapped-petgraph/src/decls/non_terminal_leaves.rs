macro_rules! deps {
    () => {
        NodeRef!();
    };
}

macro_rules! non_terminal_leaves {
    () => {
        deps!();
        fn non_terminal_leaves < G > (graph : G , terminals : & [G :: NodeId]) -> HashSet < G :: NodeId > where G : GraphBase + IntoNodeReferences + IntoNodeIdentifiers + IntoNeighbors , G :: NodeId : Hash + Eq + Debug , G :: NodeRef : Eq + Hash , { let mut removed_leaves = HashSet :: new () ; let mut remaining_leaves = graph . node_identifiers () . filter (| node_id | { graph . neighbors (* node_id) . collect :: < HashSet < _ > > () . len () == 1 && ! terminals . contains (node_id) }) . collect :: < HashSet < _ > > () ; while ! remaining_leaves . is_empty () { remaining_leaves = graph . node_identifiers () . filter (| node_id | { ! terminals . contains (node_id) && ! removed_leaves . contains (node_id) && (graph . neighbors (* node_id) . collect :: < HashSet < _ > > () . difference (& removed_leaves)) . collect :: < Vec < _ > > () . len () == 1 }) . collect :: < HashSet < _ > > () ; removed_leaves = removed_leaves . union (& remaining_leaves) . cloned () . collect :: < HashSet < _ > > () ; } removed_leaves }
    };
}

non_terminal_leaves!();
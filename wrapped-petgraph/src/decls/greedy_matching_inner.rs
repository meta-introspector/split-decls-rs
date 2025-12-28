macro_rules! greedy_matching_inner {
    () => {
        # [inline] fn greedy_matching_inner < G > (graph : & G) -> (Vec < Option < G :: NodeId > > , usize) where G : Visitable + IntoNodeIdentifiers + NodeIndexable + IntoNeighbors , { let mut mate = vec ! [None ; graph . node_bound ()] ; let mut n_edges = 0 ; let visited = & mut graph . visit_map () ; for start in graph . node_identifiers () { let mut last = Some (start) ; non_backtracking_dfs (graph , start , visited , | next | { if let Some (pred) = last . take () { mate [graph . to_index (pred)] = Some (next) ; mate [graph . to_index (next)] = Some (pred) ; n_edges += 1 ; } else { last = Some (next) ; } }) ; } (mate , n_edges) }
    };
}

greedy_matching_inner!();
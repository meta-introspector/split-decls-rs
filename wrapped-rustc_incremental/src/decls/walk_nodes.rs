macro_rules! walk_nodes {
    () => {
        fn walk_nodes < 'q > (query : & 'q DepGraphQuery , starts : & FxIndexSet < & 'q DepNode > , direction : Direction ,) -> FxIndexSet < DepKind > { let mut set = FxIndexSet :: default () ; for & start in starts { debug ! ("walk_nodes: start={:?} outgoing?={:?}" , start , direction == OUTGOING) ; if set . insert (start . kind) { let mut stack = vec ! [query . indices [start]] ; while let Some (index) = stack . pop () { for (_ , edge) in query . graph . adjacent_edges (index , direction) { let neighbor_index = edge . source_or_target (direction) ; let neighbor = query . graph . node_data (neighbor_index) ; if set . insert (neighbor . kind) { stack . push (neighbor_index) ; } } } } } set }
    };
}

walk_nodes!()
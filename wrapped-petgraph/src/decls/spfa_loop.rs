macro_rules! deps {
    () => {
        NegativeCycle!();
        EdgeRef!();
        BoundedMeasure!();
    };
}

macro_rules! spfa_loop {
    () => {
        deps!();
        # [doc = " The main cycle of the SPFA algorithm. Calculating the predecessors is optional."] # [doc = ""] # [doc = " The `queue` must be pre-initialized by at least one `source` node."] # [doc = " The content of `in_queue` must match to `queue`."] # [allow (clippy :: type_complexity)] pub (crate) fn spfa_loop < G , F , K > (graph : G , mut distances : Vec < K > , mut predecessors : Option < Vec < Option < G :: NodeId > > > , mut queue : VecDeque < G :: NodeId > , mut in_queue : Vec < bool > , mut edge_cost : F ,) -> Result < (Vec < K > , Option < Vec < Option < G :: NodeId > > >) , NegativeCycle > where G : IntoEdges + IntoNodeIdentifiers + NodeIndexable , F : FnMut (G :: EdgeRef) -> K , K : BoundedMeasure + Copy , { let ix = | i | graph . to_index (i) ; let mut visits = vec ! [0 ; graph . node_bound ()] ; while let Some (i) = queue . pop_front () { in_queue [ix (i)] = false ; if visits [ix (i)] >= graph . node_bound () { return Err (NegativeCycle (())) ; } visits [ix (i)] += 1 ; for edge in graph . edges (i) { let j = edge . target () ; let w = edge_cost (edge) ; let (dist , overflow) = distances [ix (i)] . overflowing_add (w) ; if ! overflow && dist < distances [ix (j)] { distances [ix (j)] = dist ; if let Some (p) = predecessors . as_mut () { p [ix (j)] = Some (i) } if ! in_queue [ix (j)] { in_queue [ix (j)] = true ; queue . push_back (j) ; } } } } Ok ((distances , predecessors)) }
    };
}

spfa_loop!()
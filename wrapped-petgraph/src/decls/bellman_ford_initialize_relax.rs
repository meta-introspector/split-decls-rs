macro_rules! deps {
    () => {
        FloatMeasure!();
    };
}

macro_rules! bellman_ford_initialize_relax {
    () => {
        deps!();
        # [inline (always)] fn bellman_ford_initialize_relax < G > (g : G , source : G :: NodeId ,) -> (Vec < G :: EdgeWeight > , Vec < Option < G :: NodeId > >) where G : NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable , G :: EdgeWeight : FloatMeasure , { let mut predecessor = vec ! [None ; g . node_bound ()] ; let mut distance = vec ! [< _ >:: infinite () ; g . node_bound ()] ; let ix = | i | g . to_index (i) ; distance [ix (source)] = < _ > :: zero () ; for _ in 1 .. g . node_count () { let mut did_update = false ; for i in g . node_identifiers () { for edge in g . edges (i) { let j = edge . target () ; let w = * edge . weight () ; if distance [ix (i)] + w < distance [ix (j)] { distance [ix (j)] = distance [ix (i)] + w ; predecessor [ix (j)] = Some (i) ; did_update = true ; } } } if ! did_update { break ; } } (distance , predecessor) }
    };
}

bellman_ford_initialize_relax!()
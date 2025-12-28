macro_rules! deps {
    () => {
        UnitMeasure!();
    };
}

macro_rules! out_edges_info {
    () => {
        deps!();
        # [allow (dead_code)] fn out_edges_info < G , D > (graph : G , index_w : usize , index_v : usize) -> (D , bool) where G : NodeCount + IntoEdges + NodeIndexable + core :: marker :: Sync , D : UnitMeasure + Copy + core :: marker :: Send + core :: marker :: Sync , { let node_w = graph . from_index (index_w) ; let node_v = graph . from_index (index_v) ; let mut out_edges = graph . edges (node_w) ; let mut out_edge = out_edges . next () ; let mut out_degree = D :: zero () ; let mut flag_points_to = false ; while let Some (edge) = out_edge { out_degree = out_degree + D :: one () ; if edge . target () == node_v { flag_points_to = true ; } out_edge = out_edges . next () ; } (out_degree , flag_points_to) }
    };
}

out_edges_info!()
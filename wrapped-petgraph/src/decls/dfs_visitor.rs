macro_rules! deps {
    () => {
        Time!();
        ControlFlow!();
        DfsEvent!();
        VisitMap!();
    };
}

macro_rules! dfs_visitor {
    () => {
        deps!();
        pub (crate) fn dfs_visitor < G , F , C > (graph : G , u : G :: NodeId , visitor : & mut F , discovered : & mut impl VisitMap < G :: NodeId > , finished : & mut impl VisitMap < G :: NodeId > , time : & mut Time ,) -> C where G : IntoNeighbors + Visitable , F : FnMut (DfsEvent < G :: NodeId >) -> C , C : ControlFlow , { if ! discovered . visit (u) { return C :: continuing () ; } try_control ! (visitor (DfsEvent :: Discover (u , time_post_inc (time))) , { } , for v in graph . neighbors (u) { if ! discovered . is_visited (& v) { try_control ! (visitor (DfsEvent :: TreeEdge (u , v)) , continue) ; try_control ! (dfs_visitor (graph , v , visitor , discovered , finished , time) , unreachable ! ()) ; } else if ! finished . is_visited (& v) { try_control ! (visitor (DfsEvent :: BackEdge (u , v)) , continue) ; } else { try_control ! (visitor (DfsEvent :: CrossForwardEdge (u , v)) , continue) ; } }) ; let first_finish = finished . visit (u) ; debug_assert ! (first_finish) ; try_control ! (visitor (DfsEvent :: Finish (u , time_post_inc (time))) , panic ! ("Pruning on the `DfsEvent::Finish` is not supported!")) ; C :: continuing () }
    };
}

dfs_visitor!();
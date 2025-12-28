macro_rules! deps {
    () => {
        StableGraph!();
    };
}

macro_rules! test_retain_nodes {
    () => {
        deps!();
        # [test] fn test_retain_nodes () { let mut gr = StableGraph :: < _ , _ > :: with_capacity (6 , 6) ; let a = gr . add_node ("a") ; let f = gr . add_node ("f") ; let b = gr . add_node ("b") ; let c = gr . add_node ("c") ; let d = gr . add_node ("d") ; let e = gr . add_node ("e") ; gr . add_edge (a , b , 1) ; gr . add_edge (a , c , 2) ; gr . add_edge (b , c , 3) ; gr . add_edge (b , d , 4) ; gr . add_edge (c , d , 5) ; gr . add_edge (d , b , 6) ; gr . add_edge (c , b , 7) ; gr . add_edge (d , e , 8) ; gr . remove_node (f) ; assert_eq ! (gr . node_count () , 5) ; assert_eq ! (gr . edge_count () , 8) ; gr . retain_nodes (| frozen_gr , ix | frozen_gr [ix] >= "c") ; assert_eq ! (gr . node_count () , 3) ; assert_eq ! (gr . edge_count () , 2) ; gr . check_free_lists () ; }
    };
}

test_retain_nodes!();
macro_rules! deps {
    () => {
        StableGraph!();
    };
}

macro_rules! test_reverse {
    () => {
        deps!();
        # [test] fn test_reverse () { let mut gr = StableGraph :: < _ , _ > :: default () ; let a = gr . add_node ("a") ; let b = gr . add_node ("b") ; gr . add_edge (a , b , 0) ; let mut reversed_gr = gr . clone () ; reversed_gr . reverse () ; for i in gr . node_indices () { itertools :: assert_equal (gr . edges_directed (i , Incoming) , reversed_gr . edges (i)) ; } }
    };
}

test_reverse!()
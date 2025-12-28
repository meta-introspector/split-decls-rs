macro_rules! deps {
    () => {
        StableGraph!();
    };
}

macro_rules! test_reverse_after_remove_nodes {
    () => {
        deps!();
        # [test] fn test_reverse_after_remove_nodes () { let mut gr = StableGraph :: < _ , _ > :: default () ; let a = gr . add_node ("a") ; let b = gr . add_node ("b") ; let c = gr . add_node ("c") ; gr . add_edge (a , a , 0) ; gr . remove_node (b) ; gr . remove_node (c) ; gr . check_free_lists () ; gr . reverse () ; gr . check_free_lists () ; }
    };
}

test_reverse_after_remove_nodes!()
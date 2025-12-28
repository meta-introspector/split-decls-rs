macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! test_easy_tred {
    () => {
        deps!();
        # [cfg (test)] # [test] fn test_easy_tred () { let mut input = List :: new () ; let a : u8 = input . add_node () ; let b = input . add_node () ; let c = input . add_node () ; input . add_edge (a , b , ()) ; input . add_edge (a , c , ()) ; input . add_edge (b , c , ()) ; let (tred , tclos) = dag_transitive_reduction_closure (& input) ; assert_eq ! (tred . node_count () , 3) ; assert_eq ! (tclos . node_count () , 3) ; assert ! (tred . find_edge (a , b) . is_some ()) ; assert ! (tred . find_edge (b , c) . is_some ()) ; assert ! (tred . find_edge (a , c) . is_none ()) ; assert ! (tclos . find_edge (a , b) . is_some ()) ; assert ! (tclos . find_edge (b , c) . is_some ()) ; assert ! (tclos . find_edge (a , c) . is_some ()) ; }
    };
}

test_easy_tred!();
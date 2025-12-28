macro_rules! par_iter_collect_linked_list_flat_map_filter {
    () => {
        # [test] fn par_iter_collect_linked_list_flat_map_filter () { let b : LinkedList < i32 > = (0_i32 .. 1024) . into_par_iter () . flat_map (| i | 0 .. i) . filter (| & i | i % 2 == 0) . collect () ; let c : LinkedList < i32 > = (0_i32 .. 1024) . flat_map (| i | 0 .. i) . filter (| & i | i % 2 == 0) . collect () ; assert_eq ! (b , c) ; }
    };
}

par_iter_collect_linked_list_flat_map_filter!();
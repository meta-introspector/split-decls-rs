macro_rules! par_iter_unindexed_flat_map {
    () => {
        # [test] fn par_iter_unindexed_flat_map () { let b : Vec < i64 > = (0_i64 .. 1024) . into_par_iter () . flat_map (Some) . collect () ; let c : Vec < i64 > = (0_i64 .. 1024) . flat_map (Some) . collect () ; assert_eq ! (b , c) ; }
    };
}

par_iter_unindexed_flat_map!();
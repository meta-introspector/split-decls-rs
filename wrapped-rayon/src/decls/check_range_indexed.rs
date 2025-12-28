macro_rules! check_range_indexed {
    () => {
        # [test] fn check_range_indexed () { is_indexed ((1 .. 5) . into_par_iter ()) ; }
    };
}

check_range_indexed!();
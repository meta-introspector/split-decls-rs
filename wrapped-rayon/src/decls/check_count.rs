macro_rules! check_count {
    () => {
        # [test] fn check_count () { let c0 = (0_u32 .. 24 * 1024) . filter (| i | i % 2 == 0) . count () ; let c1 = (0_u32 .. 24 * 1024) . into_par_iter () . filter (| i | i % 2 == 0) . count () ; assert_eq ! (c0 , c1) ; }
    };
}

check_count!();
macro_rules! check_step_by_unaligned {
    () => {
        # [test] fn check_step_by_unaligned () { let a : Vec < i32 > = (0 .. 1029) . step_by (10) . collect () ; let b : Vec < i32 > = (0 .. 1029) . into_par_iter () . step_by (10) . collect () ; assert_eq ! (a , b) }
    };
}

check_step_by_unaligned!()
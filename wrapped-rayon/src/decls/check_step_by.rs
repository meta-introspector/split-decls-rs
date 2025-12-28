macro_rules! check_step_by {
    () => {
        # [test] fn check_step_by () { let a : Vec < i32 > = (0 .. 1024) . step_by (2) . collect () ; let b : Vec < i32 > = (0 .. 1024) . into_par_iter () . step_by (2) . collect () ; assert_eq ! (a , b) ; }
    };
}

check_step_by!();
macro_rules! check_interleave_eq {
    () => {
        # [test] fn check_interleave_eq () { let xs : Vec < usize > = (0 .. 10) . collect () ; let ys : Vec < usize > = (10 .. 20) . collect () ; let mut actual = vec ! [] ; xs . par_iter () . interleave (& ys) . map (| & i | i) . collect_into_vec (& mut actual) ; let expected : Vec < usize > = (0 .. 10) . zip (10 .. 20) . flat_map (| (i , j) | vec ! [i , j] . into_iter ()) . collect () ; assert_eq ! (expected , actual) ; }
    };
}

check_interleave_eq!()
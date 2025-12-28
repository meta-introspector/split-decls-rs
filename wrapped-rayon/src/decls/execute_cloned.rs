macro_rules! execute_cloned {
    () => {
        # [test] fn execute_cloned () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let mut b : Vec < i32 > = vec ! [] ; a . par_iter () . cloned () . collect_into_vec (& mut b) ; let c : Vec < i32 > = (0 .. 1024) . collect () ; assert_eq ! (b , c) ; }
    };
}

execute_cloned!()
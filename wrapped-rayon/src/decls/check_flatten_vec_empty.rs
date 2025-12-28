macro_rules! check_flatten_vec_empty {
    () => {
        # [test] fn check_flatten_vec_empty () { let a : Vec < Vec < i32 > > = vec ! [vec ! []] ; let b : Vec < i32 > = a . par_iter () . flatten () . cloned () . collect () ; assert_eq ! (vec ! [] as Vec < i32 >, b) ; }
    };
}

check_flatten_vec_empty!();
macro_rules! check_cmp_lengths {
    () => {
        # [test] fn check_cmp_lengths () { let a = vec ! [0 ; 1024] ; let b = vec ! [0 ; 1025] ; assert_eq ! (a . par_iter () . cmp (& b) , a . iter () . cmp (& b)) ; assert_eq ! (a . par_iter () . partial_cmp (& b) , a . iter () . partial_cmp (& b)) ; }
    };
}

check_cmp_lengths!();
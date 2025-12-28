macro_rules! check_vec_indexed {
    () => {
        # [test] fn check_vec_indexed () { let a = vec ! [1 , 2 , 3] ; is_indexed (a . into_par_iter ()) ; }
    };
}

check_vec_indexed!();
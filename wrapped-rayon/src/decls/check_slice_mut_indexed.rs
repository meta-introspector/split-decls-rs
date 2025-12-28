macro_rules! check_slice_mut_indexed {
    () => {
        # [test] fn check_slice_mut_indexed () { let mut a = vec ! [1 , 2 , 3] ; is_indexed (a . par_iter_mut ()) ; }
    };
}

check_slice_mut_indexed!();
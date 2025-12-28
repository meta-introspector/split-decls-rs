macro_rules! check_slice_indexed {
    () => {
        # [test] fn check_slice_indexed () { let a = vec ! [1 , 2 , 3] ; is_indexed (a . par_iter ()) ; }
    };
}

check_slice_indexed!();
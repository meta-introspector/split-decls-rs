macro_rules! check_chunks_even_size {
    () => {
        # [test] fn check_chunks_even_size () { assert_eq ! (vec ! [vec ! [1 , 2 , 3] , vec ! [4 , 5 , 6] , vec ! [7 , 8 , 9]] , (1 .. 10) . into_par_iter () . chunks (3) . collect ::< Vec < Vec < i32 >>> ()) ; }
    };
}

check_chunks_even_size!();
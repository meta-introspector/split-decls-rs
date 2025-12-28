macro_rules! check_chunks_len {
    () => {
        # [test] fn check_chunks_len () { assert_eq ! (4 , (0 .. 8) . into_par_iter () . chunks (2) . len ()) ; assert_eq ! (3 , (0 .. 9) . into_par_iter () . chunks (3) . len ()) ; assert_eq ! (3 , (0 .. 8) . into_par_iter () . chunks (3) . len ()) ; assert_eq ! (1 , [1] . par_iter () . chunks (3) . len ()) ; assert_eq ! (0 , (0 .. 0) . into_par_iter () . chunks (3) . len ()) ; }
    };
}

check_chunks_len!()
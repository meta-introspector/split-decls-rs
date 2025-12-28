macro_rules! check_chunks_zero_size {
    () => {
        # [test] # [should_panic (expected = "chunk_size must not be zero")] fn check_chunks_zero_size () { let _ : Vec < Vec < i32 > > = vec ! [1 , 2 , 3] . into_par_iter () . chunks (0) . collect () ; }
    };
}

check_chunks_zero_size!();
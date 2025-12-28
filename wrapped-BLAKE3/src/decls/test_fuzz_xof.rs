macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_fuzz_xof {
    () => {
        deps!();
        # [test] fn test_fuzz_xof () { let mut input_buf = [0u8 ; 3 * BLOCK_LEN] ; paint_test_input (& mut input_buf) ; let num_tests = if cfg ! (debug_assertions) { 100 } else { 2500 } ; let mut rng = rand_chacha :: ChaCha8Rng :: from_seed ([1 ; 32]) ; for _num_test in 0 .. num_tests { # [cfg (feature = "std")] dbg ! (_num_test) ; let mut output_buf = [0 ; 31 * CHUNK_LEN] ; let input_len = rng . random_range (0 .. input_buf . len ()) ; let mut xof = crate :: Hasher :: new () . update (& input_buf [.. input_len]) . finalize_xof () ; let partial_start = rng . random_range (0 .. output_buf . len ()) ; let partial_end = rng . random_range (partial_start .. output_buf . len ()) ; xof . fill (& mut output_buf [.. partial_start]) ; xof . fill (& mut output_buf [partial_start .. partial_end]) ; xof . fill (& mut output_buf [partial_end ..]) ; let mut reference_buf = [0 ; 31 * CHUNK_LEN] ; let mut reference_hasher = reference_impl :: Hasher :: new () ; reference_hasher . update (& input_buf [.. input_len]) ; reference_hasher . finalize (& mut reference_buf) ; assert_eq ! (reference_buf , output_buf) ; } }
    };
}

test_fuzz_xof!()
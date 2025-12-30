// Generated macro for test_fuzz_hasher (function)
macro_rules! Depcrate_testtest_fuzz_hasher {
() => {
// Module: crate::test
// Provides: {"test_fuzz_hasher"}
// Dependencies: {}
# [test] fn test_fuzz_hasher () { const INPUT_MAX : usize = 4 * CHUNK_LEN ; let mut input_buf = [0 ; 3 * INPUT_MAX] ; paint_test_input (& mut input_buf) ; let num_tests = if cfg ! (debug_assertions) { 100 } else { 10_000 } ; let mut rng = rand_chacha :: ChaCha8Rng :: from_seed ([1 ; 32]) ; for _num_test in 0 .. num_tests { # [cfg (feature = "std")] dbg ! (_num_test) ; let mut hasher = crate :: Hasher :: new () ; let mut total_input = 0 ; for _ in 0 .. 3 { let input_len = rng . random_range (0 .. (INPUT_MAX + 1)) ; # [cfg (feature = "std")] dbg ! (input_len) ; let input = & input_buf [total_input ..] [.. input_len] ; hasher . update (input) ; total_input += input_len ; } let expected = reference_hash (& input_buf [.. total_input]) ; assert_eq ! (expected , hasher . finalize ()) ; } }
};
}

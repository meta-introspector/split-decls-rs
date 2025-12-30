// Generated macro for test_compare_update_multiple (function)
macro_rules! Depcrate_testtest_compare_update_multiple {
() => {
// Module: crate::test
// Provides: {"test_compare_update_multiple"}
// Dependencies: {}
# [test] fn test_compare_update_multiple () { let mut short_test_cases = TEST_CASES ; while * short_test_cases . last () . unwrap () > 4 * CHUNK_LEN { short_test_cases = & short_test_cases [.. short_test_cases . len () - 1] ; } assert_eq ! (* short_test_cases . last () . unwrap () , 4 * CHUNK_LEN) ; let mut input_buf = [0 ; 2 * TEST_CASES_MAX] ; paint_test_input (& mut input_buf) ; for & first_update in short_test_cases { # [cfg (feature = "std")] dbg ! (first_update) ; let first_input = & input_buf [.. first_update] ; let mut test_hasher = crate :: Hasher :: new () ; test_hasher . update (first_input) ; for & second_update in short_test_cases { # [cfg (feature = "std")] dbg ! (second_update) ; let second_input = & input_buf [first_update ..] [.. second_update] ; let total_input = & input_buf [.. first_update + second_update] ; let mut test_hasher = test_hasher . clone () ; test_hasher . update (second_input) ; let expected = reference_hash (total_input) ; assert_eq ! (expected , test_hasher . finalize ()) ; } } }
};
}

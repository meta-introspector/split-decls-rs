// Generated macro for count_zero (function)
macro_rules! Depcrate_multi_testscount_zero {
() => {
// Module: crate::multi::tests
// Provides: {"count_zero"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn count_zero () { const TIMES : usize = 0 ; fn counter_2 (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { count (tag ("abc") , TIMES) . parse (i) } let done = & b"abcabcabcdef" [..] ; let parsed_done = Vec :: new () ; let rest = done ; let incomplete_1 = & b"ab" [..] ; let parsed_incompl_1 = Vec :: new () ; let incomplete_2 = & b"abcab" [..] ; let parsed_incompl_2 = Vec :: new () ; let error = & b"xxx" [..] ; let error_remain = & b"xxx" [..] ; let parsed_err = Vec :: new () ; let error_1 = & b"xxxabcabcdef" [..] ; let parsed_err_1 = Vec :: new () ; let error_1_remain = & b"xxxabcabcdef" [..] ; let error_2 = & b"abcxxxabcdef" [..] ; let parsed_err_2 = Vec :: new () ; let error_2_remain = & b"abcxxxabcdef" [..] ; assert_eq ! (counter_2 (done) , Ok ((rest , parsed_done))) ; assert_eq ! (counter_2 (incomplete_1) , Ok ((incomplete_1 , parsed_incompl_1))) ; assert_eq ! (counter_2 (incomplete_2) , Ok ((incomplete_2 , parsed_incompl_2))) ; assert_eq ! (counter_2 (error) , Ok ((error_remain , parsed_err))) ; assert_eq ! (counter_2 (error_1) , Ok ((error_1_remain , parsed_err_1))) ; assert_eq ! (counter_2 (error_2) , Ok ((error_2_remain , parsed_err_2))) ; }
};
}

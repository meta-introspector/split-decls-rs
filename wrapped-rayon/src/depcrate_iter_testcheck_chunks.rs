// Generated macro for check_chunks (function)
macro_rules! Depcrate_iter_testcheck_chunks {
() => {
// Module: crate::iter::test
// Provides: {"check_chunks"}
// Dependencies: {}
# [test] fn check_chunks () { let a : Vec < i32 > = vec ! [1 , 5 , 10 , 4 , 100 , 3 , 1000 , 2 , 10000 , 1] ; let par_sum_product_pairs : i32 = a . par_chunks (2) . map (| c | c . iter () . product :: < i32 > ()) . sum () ; let seq_sum_product_pairs = a . chunks (2) . map (| c | c . iter () . product :: < i32 > ()) . sum () ; assert_eq ! (par_sum_product_pairs , 12345) ; assert_eq ! (par_sum_product_pairs , seq_sum_product_pairs) ; let par_sum_product_triples : i32 = a . par_chunks (3) . map (| c | c . iter () . product :: < i32 > ()) . sum () ; let seq_sum_product_triples = a . chunks (3) . map (| c | c . iter () . product :: < i32 > ()) . sum () ; assert_eq ! (par_sum_product_triples , 5_0 + 12_00 + 20_000_000 + 1) ; assert_eq ! (par_sum_product_triples , seq_sum_product_triples) ; }
};
}

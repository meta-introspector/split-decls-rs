// Generated macro for check_chunks_uneven (function)
macro_rules! Depcrate_iter_testcheck_chunks_uneven {
() => {
// Module: crate::iter::test
// Provides: {"check_chunks_uneven"}
// Dependencies: {}
# [test] fn check_chunks_uneven () { let cases : Vec < (Vec < u32 > , usize , Vec < Vec < u32 > >) > = vec ! [((0 .. 5) . collect () , 3 , vec ! [vec ! [0 , 1 , 2] , vec ! [3 , 4]]) , (vec ! [1] , 5 , vec ! [vec ! [1]]) , ((0 .. 4) . collect () , 3 , vec ! [vec ! [0 , 1 , 2] , vec ! [3]]) ,] ; for (i , (v , n , expected)) in cases . into_iter () . enumerate () { let mut res : Vec < Vec < u32 > > = vec ! [] ; v . par_iter () . chunks (n) . map (| v | v . into_iter () . cloned () . collect ()) . collect_into_vec (& mut res) ; assert_eq ! (expected , res , "Case {i} failed") ; res . truncate (0) ; v . into_par_iter () . chunks (n) . rev () . collect_into_vec (& mut res) ; assert_eq ! (expected . into_iter () . rev () . collect ::< Vec < Vec < u32 >>> () , res , "Case {i} reversed failed") ; } }
};
}

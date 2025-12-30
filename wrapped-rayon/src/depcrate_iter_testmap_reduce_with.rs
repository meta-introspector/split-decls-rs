// Generated macro for map_reduce_with (function)
macro_rules! Depcrate_iter_testmap_reduce_with {
() => {
// Module: crate::iter::test
// Provides: {"map_reduce_with"}
// Dependencies: {}
# [test] fn map_reduce_with () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let r1 = a . par_iter () . map (| & i | i + 1) . reduce_with (| i , j | i + j) ; let r2 = a . iter () . map (| & i | i + 1) . sum () ; assert_eq ! (r1 , Some (r2)) ; }
};
}

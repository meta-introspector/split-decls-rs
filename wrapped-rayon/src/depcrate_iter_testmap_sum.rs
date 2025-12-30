// Generated macro for map_sum (function)
macro_rules! Depcrate_iter_testmap_sum {
() => {
// Module: crate::iter::test
// Provides: {"map_sum"}
// Dependencies: {}
# [test] fn map_sum () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let r1 : i32 = a . par_iter () . map (| & i | i + 1) . sum () ; let r2 = a . iter () . map (| & i | i + 1) . sum () ; assert_eq ! (r1 , r2) ; }
};
}

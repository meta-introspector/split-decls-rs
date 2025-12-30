// Generated macro for max_by (function)
macro_rules! Depcrate_testmax_by {
() => {
// Module: crate::test
// Provides: {"max_by"}
// Dependencies: {}
# [test] fn max_by () { let it = convert (vec ! [0 , 3 , 1 , - 10] . into_iter () . map (Ok :: < i32 , () >)) ; assert_eq ! (it . max_by (| a , b | Ok (b . cmp (a))) , Ok (Some (- 10))) ; }
};
}

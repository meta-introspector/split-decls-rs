// Generated macro for find_map (function)
macro_rules! Depcrate_testfind_map {
() => {
// Module: crate::test
// Provides: {"find_map"}
// Dependencies: {}
# [test] fn find_map () { let mut it = convert (vec ! [0 , 1 , 2 , 3] . into_iter () . map (Ok :: < i32 , () >)) ; assert_eq ! (it . find_map (| v | match v { 2 => Ok (Some ("hi")) , _ => Ok (None) , }) , Ok (Some ("hi"))) ; }
};
}

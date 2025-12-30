// Generated macro for for_each (function)
macro_rules! Depcrate_testfor_each {
() => {
// Module: crate::test
// Provides: {"for_each"}
// Dependencies: {}
# [test] fn for_each () { let it = convert (vec ! [0 , 1 , 2 , 3] . into_iter () . map (Ok :: < u32 , () >)) ; let mut acc = vec ! [] ; it . for_each (| n | { acc . push (n) ; Ok (()) }) . unwrap () ; assert_eq ! (acc , vec ! [0 , 1 , 2 , 3]) ; }
};
}

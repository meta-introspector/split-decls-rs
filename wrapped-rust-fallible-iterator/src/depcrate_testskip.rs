// Generated macro for skip (function)
macro_rules! Depcrate_testskip {
() => {
// Module: crate::test
// Provides: {"skip"}
// Dependencies: {}
# [test] fn skip () { let it = convert (vec ! [1 , 2 , 3 , 4] . into_iter () . map (Ok :: < i32 , () >)) ; assert_eq ! (it . clone () . skip (0) . collect ::< Vec < _ >> () , Ok (vec ! [1 , 2 , 3 , 4])) ; assert_eq ! (it . clone () . skip (2) . collect ::< Vec < _ >> () , Ok (vec ! [3 , 4])) ; assert_eq ! (it . skip (4) . collect ::< Vec < _ >> () , Ok (vec ! [])) ; }
};
}

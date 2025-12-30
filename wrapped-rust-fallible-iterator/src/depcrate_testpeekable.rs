// Generated macro for peekable (function)
macro_rules! Depcrate_testpeekable {
() => {
// Module: crate::test
// Provides: {"peekable"}
// Dependencies: {}
# [test] fn peekable () { let mut it = convert (vec ! [0 , 1] . into_iter () . map (Ok :: < i32 , () >)) . peekable () ; assert_eq ! (it . peek () . unwrap () , Some (& 0)) ; assert_eq ! (it . peek () . unwrap () , Some (& 0)) ; assert_eq ! (it . next () . unwrap () , Some (0)) ; assert_eq ! (it . next () . unwrap () , Some (1)) ; assert_eq ! (it . peek () . unwrap () , None) ; assert_eq ! (it . next () . unwrap () , None) ; }
};
}

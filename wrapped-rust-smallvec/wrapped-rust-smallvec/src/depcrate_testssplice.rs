// Generated macro for splice (function)
macro_rules! Depcrate_testssplice {
() => {
// Module: crate::tests
// Provides: {"splice"}
// Dependencies: {}
# [test] fn splice () { let mut v : SmallVec < u8 , 1 > = smallvec ! [0 , 1 , 2 , 3 , 4 , 5 , 6] ; let new = [7 , 8 , 9 , 10] ; let u : SmallVec < u8 , 1 > = v . splice (6 .. , new) . collect () ; assert_eq ! (v , [0 , 1 , 2 , 3 , 4 , 5 , 7 , 8 , 9 , 10]) ; assert_eq ! (u , [6]) ; let mut v : SmallVec < u8 , 1 > = smallvec ! [0 , 1 , 2 , 3 , 4 , 5 , 6] ; let new = [7 , 8 , 9 , 10] ; let u : SmallVec < u8 , 1 > = v . splice (1 .. 1 , new) . collect () ; assert_eq ! (v , [0 , 7 , 8 , 9 , 10 , 1 , 2 , 3 , 4 , 5 , 6]) ; assert_eq ! (u , []) ; let mut v : SmallVec < u8 , 1 > = smallvec ! [0 , 1 , 2 , 3 , 4 , 5 , 6] ; let new = [7 , 8 , 9 , 10] ; let u : SmallVec < u8 , 1 > = v . splice (.. 3 , new) . collect () ; assert_eq ! (v , [7 , 8 , 9 , 10 , 3 , 4 , 5 , 6]) ; assert_eq ! (u , [0 , 1 , 2]) ; }
};
}

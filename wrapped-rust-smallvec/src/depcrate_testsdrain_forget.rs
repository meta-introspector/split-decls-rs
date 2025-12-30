// Generated macro for drain_forget (function)
macro_rules! Depcrate_testsdrain_forget {
() => {
// Module: crate::tests
// Provides: {"drain_forget"}
// Dependencies: {}
# [test] fn drain_forget () { let mut v : SmallVec < u8 , 1 > = smallvec ! [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] ; std :: mem :: forget (v . drain (2 .. 5)) ; assert_eq ! (v . len () , 2) ; }
};
}

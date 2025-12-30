// Generated macro for drain_rev (function)
macro_rules! Depcrate_testsdrain_rev {
() => {
// Module: crate::tests
// Provides: {"drain_rev"}
// Dependencies: {}
# [test] fn drain_rev () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; assert_eq ! (v . drain (..) . rev () . collect ::< Vec < _ >> () , & [3]) ; v . push (3) ; v . push (4) ; v . push (5) ; assert_eq ! (v . drain (..) . rev () . collect ::< Vec < _ >> () , & [5 , 4 , 3]) ; }
};
}

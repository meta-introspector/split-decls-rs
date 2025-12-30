// Generated macro for grow_to_shrink (function)
macro_rules! Depcrate_testsgrow_to_shrink {
() => {
// Module: crate::tests
// Provides: {"grow_to_shrink"}
// Dependencies: {}
# [test] fn grow_to_shrink () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (1) ; v . push (2) ; v . push (3) ; assert ! (v . spilled ()) ; v . clear () ; v . grow (2) ; assert ! (! v . spilled ()) ; assert_eq ! (v . capacity () , 2) ; assert_eq ! (v . len () , 0) ; v . push (4) ; assert_eq ! (v [..] , [4]) ; }
};
}

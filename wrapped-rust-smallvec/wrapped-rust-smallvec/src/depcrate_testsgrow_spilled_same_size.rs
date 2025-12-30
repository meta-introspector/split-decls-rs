// Generated macro for grow_spilled_same_size (function)
macro_rules! Depcrate_testsgrow_spilled_same_size {
() => {
// Module: crate::tests
// Provides: {"grow_spilled_same_size"}
// Dependencies: {}
# [test] fn grow_spilled_same_size () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (0) ; v . push (1) ; v . push (2) ; assert ! (v . spilled ()) ; assert_eq ! (v . capacity () , 4) ; v . grow (4) ; assert_eq ! (v . capacity () , 4) ; assert_eq ! (v [..] , [0 , 1 , 2]) ; }
};
}

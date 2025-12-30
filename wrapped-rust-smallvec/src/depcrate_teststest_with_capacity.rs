// Generated macro for test_with_capacity (function)
macro_rules! Depcrate_teststest_with_capacity {
() => {
// Module: crate::tests
// Provides: {"test_with_capacity"}
// Dependencies: {}
# [test] fn test_with_capacity () { let v : SmallVec < u8 , 3 > = SmallVec :: with_capacity (1) ; assert ! (v . is_empty ()) ; assert ! (! v . spilled ()) ; assert_eq ! (v . capacity () , 3) ; let v : SmallVec < u8 , 3 > = SmallVec :: with_capacity (10) ; assert ! (v . is_empty ()) ; assert ! (v . spilled ()) ; assert_eq ! (v . capacity () , 10) ; }
};
}

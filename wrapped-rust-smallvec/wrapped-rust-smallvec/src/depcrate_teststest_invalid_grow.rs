// Generated macro for test_invalid_grow (function)
macro_rules! Depcrate_teststest_invalid_grow {
() => {
// Module: crate::tests
// Provides: {"test_invalid_grow"}
// Dependencies: {}
# [test] # [should_panic] fn test_invalid_grow () { let mut v : SmallVec < u8 , 8 > = SmallVec :: new () ; v . extend (0 .. 8) ; v . grow (5) ; }
};
}

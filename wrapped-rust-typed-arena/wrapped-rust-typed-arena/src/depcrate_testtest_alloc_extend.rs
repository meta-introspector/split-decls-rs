// Generated macro for test_alloc_extend (function)
macro_rules! Depcrate_testtest_alloc_extend {
() => {
// Module: crate::test
// Provides: {"test_alloc_extend"}
// Dependencies: {}
# [test] fn test_alloc_extend () { let arena = Arena :: with_capacity (2) ; for i in 0 .. 15 { let slice = arena . alloc_extend (0 .. i) ; for (j , & elem) in slice . iter () . enumerate () { assert_eq ! (j , elem) ; } } }
};
}

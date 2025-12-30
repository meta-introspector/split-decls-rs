// Generated macro for test_vec (function)
macro_rules! Depcrate_teststest_vec {
() => {
// Module: crate::tests
// Provides: {"test_vec"}
// Dependencies: {}
# [test] fn test_vec () { let mut blink_alloc = BlinkAlloc :: new () ; let mut vec = Vec :: new_in (& blink_alloc) ; vec . extend ([1 , 2 , 3]) ; vec . push (4) ; vec . extend (5 .. 6) ; vec . push (6) ; assert_eq ! (vec , [1 , 2 , 3 , 4 , 5 , 6]) ; drop (vec) ; blink_alloc . reset () ; }
};
}

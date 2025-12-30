// Generated macro for test_from_array (function)
macro_rules! Depcrate_collections_vec_deque_teststest_from_array {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_from_array"}
// Dependencies: {}
# [test] fn test_from_array () { fn test < const N : usize > () { let mut array : [usize ; N] = [0 ; N] ; for i in 0 .. N { array [i] = i ; } let deq : VecDeque < _ > = array . into () ; for i in 0 .. N { assert_eq ! (deq [i] , i) ; } assert_eq ! (deq . len () , N) ; } test :: < 0 > () ; test :: < 1 > () ; test :: < 2 > () ; test :: < 32 > () ; test :: < 35 > () ; }
};
}

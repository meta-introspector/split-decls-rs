// Generated macro for make_contiguous_big_tail (function)
macro_rules! Depcrate_collections_vec_deque_testsmake_contiguous_big_tail {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"make_contiguous_big_tail"}
// Dependencies: {}
# [test] fn make_contiguous_big_tail () { let mut tester = VecDeque :: with_capacity (15) ; for i in 0 .. 8 { tester . push_back (i) ; } for i in 8 .. 10 { tester . push_front (i) ; } let expected_start = 0 ; tester . make_contiguous () ; assert_eq ! (tester . head , expected_start) ; assert_eq ! ((& [9 , 8 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] as & [_] , & [] as & [_]) , tester . as_slices ()) ; }
};
}

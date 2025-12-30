// Generated macro for make_contiguous_big_head (function)
macro_rules! Depcrate_collections_vec_deque_testsmake_contiguous_big_head {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"make_contiguous_big_head"}
// Dependencies: {}
# [test] fn make_contiguous_big_head () { let mut tester = VecDeque :: with_capacity (15) ; for i in 0 .. 3 { tester . push_back (i) ; } for i in 3 .. 10 { tester . push_front (i) ; } assert_eq ! (tester . capacity () , 15) ; assert_eq ! ((& [9 , 8 , 7 , 6 , 5 , 4 , 3] as & [_] , & [0 , 1 , 2] as & [_]) , tester . as_slices ()) ; let expected_start = tester . as_slices () . 1 . len () ; tester . make_contiguous () ; assert_eq ! (tester . head , expected_start) ; assert_eq ! ((& [9 , 8 , 7 , 6 , 5 , 4 , 3 , 0 , 1 , 2] as & [_] , & [] as & [_]) , tester . as_slices ()) ; }
};
}

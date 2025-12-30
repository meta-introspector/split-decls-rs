// Generated macro for make_contiguous_head_to_end_2 (function)
macro_rules! Depcrate_collections_vec_deque_testsmake_contiguous_head_to_end_2 {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"make_contiguous_head_to_end_2"}
// Dependencies: {}
# [test] fn make_contiguous_head_to_end_2 () { let mut dq = VecDeque :: from_iter (0 .. 6) ; dq . pop_front () ; dq . pop_front () ; dq . push_back (6) ; dq . push_back (7) ; dq . push_back (8) ; dq . make_contiguous () ; let collected : Vec < _ > = dq . iter () . copied () . collect () ; assert_eq ! (dq . as_slices () , (& collected [..] , & [] as & [_])) ; }
};
}

// Generated macro for test_vec_deque_truncate_drop (function)
macro_rules! Depcrate_collections_vec_deque_teststest_vec_deque_truncate_drop {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_vec_deque_truncate_drop"}
// Dependencies: {}
# [test] fn test_vec_deque_truncate_drop () { struct_with_counted_drop ! (Elem , DROPS) ; const LEN : usize = 5 ; for push_front in 0 ..= LEN { let mut tester = VecDeque :: with_capacity (LEN) ; for index in 0 .. LEN { if index < push_front { tester . push_front (Elem) ; } else { tester . push_back (Elem) ; } } assert_eq ! (DROPS . get () , 0) ; tester . truncate (3) ; assert_eq ! (DROPS . get () , 2) ; tester . truncate (0) ; assert_eq ! (DROPS . get () , 5) ; DROPS . set (0) ; } }
};
}

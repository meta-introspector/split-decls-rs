// Generated macro for test_swap_front_back_remove (function)
macro_rules! Depcrate_collections_vec_deque_teststest_swap_front_back_remove {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_swap_front_back_remove"}
// Dependencies: {}
# [test] fn test_swap_front_back_remove () { fn test (back : bool) { let mut tester = VecDeque :: with_capacity (15) ; let usable_cap = tester . capacity () ; let final_len = usable_cap / 2 ; for len in 0 .. final_len { let expected : VecDeque < _ > = if back { (0 .. len) . collect () } else { (0 .. len) . rev () . collect () } ; for head_pos in 0 .. usable_cap { tester . head = head_pos ; tester . len = 0 ; if back { for i in 0 .. len * 2 { tester . push_front (i) ; } for i in 0 .. len { assert_eq ! (tester . swap_remove_back (i) , Some (len * 2 - 1 - i)) ; } } else { for i in 0 .. len * 2 { tester . push_back (i) ; } for i in 0 .. len { let idx = tester . len () - 1 - i ; assert_eq ! (tester . swap_remove_front (idx) , Some (len * 2 - 1 - i)) ; } } assert ! (tester . head <= tester . capacity ()) ; assert ! (tester . len <= tester . capacity ()) ; assert_eq ! (tester , expected) ; } } } test (true) ; test (false) ; }
};
}

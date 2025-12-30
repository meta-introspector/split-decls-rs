// Generated macro for test_range_mut (function)
macro_rules! Depcrate_collections_vec_deque_teststest_range_mut {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_range_mut"}
// Dependencies: {}
# [test] fn test_range_mut () { let mut tester : VecDeque < usize > = VecDeque :: with_capacity (7) ; let cap = tester . capacity () ; for len in 0 ..= cap { for head in 0 ..= cap { for start in 0 ..= len { for end in start ..= len { tester . head = head ; tester . len = 0 ; for i in 0 .. len { tester . push_back (i) ; } let head_was = tester . head ; let len_was = tester . len ; let range : VecDeque < _ > = tester . range_mut (start .. end) . map (| v | * v) . collect () ; let expected : VecDeque < _ > = (start .. end) . collect () ; assert_eq ! (range , expected) ; assert_eq ! (tester . capacity () , cap) ; assert_eq ! (tester . head , head_was) ; assert_eq ! (tester . len , len_was) ; } } } } }
};
}

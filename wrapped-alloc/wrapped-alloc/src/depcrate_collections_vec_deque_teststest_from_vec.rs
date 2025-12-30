// Generated macro for test_from_vec (function)
macro_rules! Depcrate_collections_vec_deque_teststest_from_vec {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_from_vec"}
// Dependencies: {}
# [test] fn test_from_vec () { use crate :: vec :: Vec ; for cap in 0 .. 35 { for len in 0 ..= cap { let mut vec = Vec :: with_capacity (cap) ; vec . extend (0 .. len) ; let vd = VecDeque :: from (vec . clone ()) ; assert_eq ! (vd . len () , vec . len ()) ; assert ! (vd . into_iter () . eq (vec)) ; } } }
};
}

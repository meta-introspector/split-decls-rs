// Generated macro for test_vec_from_vecdeque (function)
macro_rules! Depcrate_collections_vec_deque_teststest_vec_from_vecdeque {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_vec_from_vecdeque"}
// Dependencies: {}
# [test] fn test_vec_from_vecdeque () { use crate :: vec :: Vec ; fn create_vec_and_test_convert (capacity : usize , offset : usize , len : usize) { let mut vd = VecDeque :: with_capacity (capacity) ; for _ in 0 .. offset { vd . push_back (0) ; vd . pop_front () ; } vd . extend (0 .. len) ; let vec : Vec < _ > = Vec :: from (vd . clone ()) ; assert_eq ! (vec . len () , vd . len ()) ; assert ! (vec . into_iter () . eq (vd)) ; } let max_pwr = if cfg ! (miri) { 5 } else { 7 } ; for cap_pwr in 0 .. max_pwr { let cap = (2i32 . pow (cap_pwr) - 1) as usize ; for len in 0 .. ((cap + 1) / 2) { for offset in 0 .. (cap - len) { create_vec_and_test_convert (cap , offset , len) } for offset in (cap - len) .. (cap - (len / 2)) { create_vec_and_test_convert (cap , offset , len) } for offset in (cap - (len / 2)) .. cap { create_vec_and_test_convert (cap , offset , len) } } for len in ((cap + 1) / 2) .. cap { for offset in 0 .. (cap - len) { create_vec_and_test_convert (cap , offset , len) } for offset in (cap - len) .. (cap - (len / 2)) { create_vec_and_test_convert (cap , offset , len) } for offset in (cap - (len / 2)) .. cap { create_vec_and_test_convert (cap , offset , len) } } } }
};
}

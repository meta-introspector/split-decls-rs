// Generated macro for issue_53529 (function)
macro_rules! Depcrate_collections_vec_deque_testsissue_53529 {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"issue_53529"}
// Dependencies: {}
# [test] fn issue_53529 () { use crate :: boxed :: Box ; let mut dst = VecDeque :: new () ; dst . push_front (Box :: new (1)) ; dst . push_front (Box :: new (2)) ; assert_eq ! (* dst . pop_back () . unwrap () , 1) ; let mut src = VecDeque :: new () ; src . push_front (Box :: new (2)) ; dst . append (& mut src) ; for a in dst { assert_eq ! (* a , 2) ; } }
};
}

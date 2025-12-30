// Generated macro for test_clone_from (function)
macro_rules! Depcrate_collections_vec_deque_teststest_clone_from {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"test_clone_from"}
// Dependencies: {}
# [test] fn test_clone_from () { let m = vec ! [1 ; 8] ; let n = vec ! [2 ; 12] ; let limit = if cfg ! (miri) { 4 } else { 8 } ; for pfv in 0 .. limit { for pfu in 0 .. limit { for longer in 0 .. 2 { let (vr , ur) = if longer == 0 { (& m , & n) } else { (& n , & m) } ; let mut v = VecDeque :: from (vr . clone ()) ; for _ in 0 .. pfv { v . push_front (1) ; } let mut u = VecDeque :: from (ur . clone ()) ; for _ in 0 .. pfu { u . push_front (2) ; } v . clone_from (& u) ; assert_eq ! (& v , & u) ; } } } }
};
}

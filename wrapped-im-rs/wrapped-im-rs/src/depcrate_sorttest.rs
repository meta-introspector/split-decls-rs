// Generated macro for test (module)
macro_rules! Depcrate_sorttest {
() => {
// Module: crate::sort
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: test :: is_sorted ; use crate :: vector :: proptest :: vector ; use :: proptest :: num :: i32 ; use :: proptest :: proptest ; proptest ! { # [test] fn test_quicksort (ref input in vector (i32 :: ANY , 0 .. 10000)) { let mut vec = input . clone () ; let len = vec . len () ; if len > 1 { quicksort (vec . focus_mut () , & Ord :: cmp) ; } assert ! (is_sorted (vec)) ; } } }
};
}

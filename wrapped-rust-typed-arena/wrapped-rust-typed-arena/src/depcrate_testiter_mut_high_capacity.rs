// Generated macro for iter_mut_high_capacity (function)
macro_rules! Depcrate_testiter_mut_high_capacity {
() => {
// Module: crate::test
// Provides: {"iter_mut_high_capacity"}
// Dependencies: {}
# [test] fn iter_mut_high_capacity () { # [derive (Debug , PartialEq , Eq)] struct NonCopy (usize) ; const MAX : usize = 1_000 ; const CAP : usize = 8192 ; let mut arena = Arena :: with_capacity (CAP) ; for i in 1 .. MAX { arena . alloc (NonCopy (i)) ; } assert ! (arena . chunks . borrow () . rest . is_empty () , "expected single chunk") ; let mut iter = arena . iter_mut () ; for i in 1 .. MAX { assert_eq ! (Some (& mut NonCopy (i)) , iter . next ()) ; } assert_eq ! (None , iter . next ()) ; }
};
}

// Generated macro for size_hint (function)
macro_rules! Depcrate_testsize_hint {
() => {
// Module: crate::test
// Provides: {"size_hint"}
// Dependencies: {}
# [test] fn size_hint () { # [derive (Debug , PartialEq , Eq)] struct NonCopy (usize) ; const MAX : usize = 32 ; const CAP : usize = 0 ; for cap in CAP .. (CAP + 16) { let mut arena = Arena :: with_capacity (cap) ; for i in 1 .. MAX { arena . alloc (NonCopy (i)) ; let iter = arena . iter_mut () ; assert_size_hint (i , iter) ; } } }
};
}

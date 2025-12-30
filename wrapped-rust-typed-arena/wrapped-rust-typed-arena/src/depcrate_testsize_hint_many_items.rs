// Generated macro for size_hint_many_items (function)
macro_rules! Depcrate_testsize_hint_many_items {
() => {
// Module: crate::test
// Provides: {"size_hint_many_items"}
// Dependencies: {}
# [test] fn size_hint_many_items () { # [derive (Debug , PartialEq , Eq)] struct NonCopy (usize) ; # [cfg (miri)] const MAX : usize = 500 ; # [cfg (not (miri))] const MAX : usize = 5_000_000 ; const CAP : usize = 16 ; let mut arena = Arena :: with_capacity (CAP) ; for i in 1 .. MAX { arena . alloc (NonCopy (i)) ; let iter = arena . iter_mut () ; assert_size_hint (i , iter) ; } }
};
}

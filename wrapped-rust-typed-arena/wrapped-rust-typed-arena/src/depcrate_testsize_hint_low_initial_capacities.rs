// Generated macro for size_hint_low_initial_capacities (function)
macro_rules! Depcrate_testsize_hint_low_initial_capacities {
() => {
// Module: crate::test
// Provides: {"size_hint_low_initial_capacities"}
// Dependencies: {}
# [test] fn size_hint_low_initial_capacities () { # [derive (Debug , PartialEq , Eq)] struct NonCopy (usize) ; # [cfg (miri)] const MAX : usize = 100 ; # [cfg (not (miri))] const MAX : usize = 25_000 ; const CAP : usize = 0 ; for cap in CAP .. (CAP + 128) { let mut arena = Arena :: with_capacity (cap) ; for i in 1 .. MAX { arena . alloc (NonCopy (i)) ; let iter = arena . iter_mut () ; assert_size_hint (i , iter) ; } } }
};
}

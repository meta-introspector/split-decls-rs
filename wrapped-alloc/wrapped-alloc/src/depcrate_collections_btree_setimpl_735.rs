// Generated macro for impl_735 (impl)
macro_rules! Depcrate_collections_btree_setimpl_735 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_735"}
// Dependencies: {}
# [stable (feature = "std_collections_from_array" , since = "1.56.0")] impl < T : Ord , const N : usize > From < [T ; N] > for BTreeSet < T > { # [doc = " Converts a `[T; N]` into a `BTreeSet<T>`."] # [doc = ""] # [doc = " If the array contains any equal values,"] # [doc = " all but one will be dropped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BTreeSet;"] # [doc = ""] # [doc = " let set1 = BTreeSet::from([1, 2, 3, 4]);"] # [doc = " let set2: BTreeSet<_> = [1, 2, 3, 4].into();"] # [doc = " assert_eq!(set1, set2);"] # [doc = " ```"] fn from (mut arr : [T ; N]) -> Self { if N == 0 { return BTreeSet :: new () ; } arr . sort () ; BTreeSet :: from_sorted_iter (IntoIterator :: into_iter (arr) , Global) } }
};
}

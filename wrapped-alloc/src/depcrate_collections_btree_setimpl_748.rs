// Generated macro for impl_748 (impl)
macro_rules! Depcrate_collections_btree_setimpl_748 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_748"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord + Clone , A : Allocator + Clone > BitOr < & BTreeSet < T , A > > for & BTreeSet < T , A > { type Output = BTreeSet < T , A > ; # [doc = " Returns the union of `self` and `rhs` as a new `BTreeSet<T>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BTreeSet;"] # [doc = ""] # [doc = " let a = BTreeSet::from([1, 2, 3]);"] # [doc = " let b = BTreeSet::from([3, 4, 5]);"] # [doc = ""] # [doc = " let result = &a | &b;"] # [doc = " assert_eq!(result, BTreeSet::from([1, 2, 3, 4, 5]));"] # [doc = " ```"] fn bitor (self , rhs : & BTreeSet < T , A >) -> BTreeSet < T , A > { BTreeSet :: from_sorted_iter (self . union (rhs) . cloned () , ManuallyDrop :: into_inner (self . map . alloc . clone ()) ,) } }
};
}

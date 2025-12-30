// Generated macro for impl_747 (impl)
macro_rules! Depcrate_collections_btree_setimpl_747 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_747"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord + Clone , A : Allocator + Clone > BitAnd < & BTreeSet < T , A > > for & BTreeSet < T , A > { type Output = BTreeSet < T , A > ; # [doc = " Returns the intersection of `self` and `rhs` as a new `BTreeSet<T>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BTreeSet;"] # [doc = ""] # [doc = " let a = BTreeSet::from([1, 2, 3]);"] # [doc = " let b = BTreeSet::from([2, 3, 4]);"] # [doc = ""] # [doc = " let result = &a & &b;"] # [doc = " assert_eq!(result, BTreeSet::from([2, 3]));"] # [doc = " ```"] fn bitand (self , rhs : & BTreeSet < T , A >) -> BTreeSet < T , A > { BTreeSet :: from_sorted_iter (self . intersection (rhs) . cloned () , ManuallyDrop :: into_inner (self . map . alloc . clone ()) ,) } }
};
}

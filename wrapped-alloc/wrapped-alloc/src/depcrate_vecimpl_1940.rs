// Generated macro for impl_1940 (impl)
macro_rules! Depcrate_vecimpl_1940 {
() => {
// Module: crate::vec
// Provides: {"impl_1940"}
// Dependencies: {}
impl < T : PartialEq , A : Allocator > Vec < T , A > { # [doc = " Removes consecutive repeated elements in the vector according to the"] # [doc = " [`PartialEq`] trait implementation."] # [doc = ""] # [doc = " If the vector is sorted, this removes all duplicates."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let mut vec = vec![1, 2, 2, 3, 2];"] # [doc = ""] # [doc = " vec.dedup();"] # [doc = ""] # [doc = " assert_eq!(vec, [1, 2, 3, 2]);"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [inline] pub fn dedup (& mut self) { self . dedup_by (| a , b | a == b) } }
};
}

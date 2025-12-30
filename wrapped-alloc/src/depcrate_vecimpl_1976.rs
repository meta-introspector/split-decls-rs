// Generated macro for impl_1976 (impl)
macro_rules! Depcrate_vecimpl_1976 {
() => {
// Module: crate::vec
// Provides: {"impl_1976"}
// Dependencies: {}
# [stable (feature = "vec_from_box" , since = "1.18.0")] impl < T , A : Allocator > From < Box < [T] , A > > for Vec < T , A > { # [doc = " Converts a boxed slice into a vector by transferring ownership of"] # [doc = " the existing heap allocation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();"] # [doc = " assert_eq!(Vec::from(b), vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : Box < [T] , A >) -> Self { s . into_vec () } }
};
}

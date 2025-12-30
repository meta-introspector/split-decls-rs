// Generated macro for impl_272 (impl)
macro_rules! Depcrate_vecimpl_272 {
() => {
// Module: crate::vec
// Provides: {"impl_272"}
// Dependencies: {}
impl < T , A : Allocator > From < Box < [T] , A > > for Vec < T , A > { # [doc = " Convert a boxed slice into a vector by transferring ownership of"] # [doc = " the existing heap allocation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::{vec, vec::Vec, boxed::Box};"] # [doc = ""] # [doc = " let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();"] # [doc = " assert_eq!(Vec::from(b), vec![1, 2, 3]);"] # [doc = " ```"] # [inline (always)] fn from (s : Box < [T] , A >) -> Self { s . into_vec () } }
};
}

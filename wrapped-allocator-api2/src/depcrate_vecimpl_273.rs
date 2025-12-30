// Generated macro for impl_273 (impl)
macro_rules! Depcrate_vecimpl_273 {
() => {
// Module: crate::vec
// Provides: {"impl_273"}
// Dependencies: {}
impl < T , A : Allocator , const N : usize > From < Box < [T ; N] , A > > for Vec < T , A > { # [doc = " Convert a boxed array into a vector by transferring ownership of"] # [doc = " the existing heap allocation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::{vec, vec::Vec, boxed::Box};"] # [doc = ""] # [doc = " let b: Box<[i32; 3]> = Box::new([1, 2, 3]);"] # [doc = " assert_eq!(Vec::from(b), vec![1, 2, 3]);"] # [doc = " ```"] # [inline (always)] fn from (s : Box < [T ; N] , A >) -> Self { s . into_vec () } }
};
}

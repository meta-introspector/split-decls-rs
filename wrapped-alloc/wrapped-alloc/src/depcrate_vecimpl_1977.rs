// Generated macro for impl_1977 (impl)
macro_rules! Depcrate_vecimpl_1977 {
() => {
// Module: crate::vec
// Provides: {"impl_1977"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "box_from_vec" , since = "1.20.0")] impl < T , A : Allocator > From < Vec < T , A > > for Box < [T] , A > { # [doc = " Converts a vector into a boxed slice."] # [doc = ""] # [doc = " Before doing the conversion, this method discards excess capacity like [`Vec::shrink_to_fit`]."] # [doc = ""] # [doc = " [owned slice]: Box"] # [doc = " [`Vec::shrink_to_fit`]: Vec::shrink_to_fit"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(Box::from(vec![1, 2, 3]), vec![1, 2, 3].into_boxed_slice());"] # [doc = " ```"] # [doc = ""] # [doc = " Any excess capacity is removed:"] # [doc = " ```"] # [doc = " let mut vec = Vec::with_capacity(10);"] # [doc = " vec.extend([1, 2, 3]);"] # [doc = ""] # [doc = " assert_eq!(Box::from(vec), vec![1, 2, 3].into_boxed_slice());"] # [doc = " ```"] # [track_caller] fn from (v : Vec < T , A >) -> Self { v . into_boxed_slice () } }
};
}

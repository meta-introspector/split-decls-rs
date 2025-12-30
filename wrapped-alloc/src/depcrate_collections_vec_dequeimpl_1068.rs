// Generated macro for impl_1068 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1068 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1068"}
// Dependencies: {}
# [stable (feature = "vecdeque_vec_conversions" , since = "1.10.0")] impl < T , A : Allocator > From < Vec < T , A > > for VecDeque < T , A > { # [doc = " Turn a [`Vec<T>`] into a [`VecDeque<T>`]."] # [doc = ""] # [doc = " [`Vec<T>`]: crate::vec::Vec"] # [doc = " [`VecDeque<T>`]: crate::collections::VecDeque"] # [doc = ""] # [doc = " This conversion is guaranteed to run in *O*(1) time"] # [doc = " and to not re-allocate the `Vec`'s buffer or allocate"] # [doc = " any additional memory."] # [inline] fn from (other : Vec < T , A >) -> Self { let (ptr , len , cap , alloc) = other . into_raw_parts_with_alloc () ; Self { head : 0 , len , buf : unsafe { RawVec :: from_raw_parts_in (ptr , cap , alloc) } } } }
};
}

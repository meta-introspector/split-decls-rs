// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_vec_deque_into_iterIntoIter {
() => {
// Module: crate::collections::vec_deque::into_iter
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the elements of a `VecDeque`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`VecDeque`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: VecDeque::into_iter"] # [derive (Clone)] # [stable (feature = "rust1" , since = "1.0.0")] pub struct IntoIter < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { inner : VecDeque < T , A > , }
};
}

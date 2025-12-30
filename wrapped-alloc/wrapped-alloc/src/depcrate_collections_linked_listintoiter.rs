// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_linked_listIntoIter {
() => {
// Module: crate::collections::linked_list
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the elements of a `LinkedList`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_iter`] method on [`LinkedList`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: LinkedList::into_iter"] # [derive (Clone)] # [stable (feature = "rust1" , since = "1.0.0")] pub struct IntoIter < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { list : LinkedList < T , A > , }
};
}

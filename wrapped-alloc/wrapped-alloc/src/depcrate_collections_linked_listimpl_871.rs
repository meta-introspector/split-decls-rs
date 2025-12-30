// Generated macro for impl_871 (impl)
macro_rules! Depcrate_collections_linked_listimpl_871 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_871"}
// Dependencies: {}
# [stable (feature = "std_collections_from_array" , since = "1.56.0")] impl < T , const N : usize > From < [T ; N] > for LinkedList < T > { # [doc = " Converts a `[T; N]` into a `LinkedList<T>`."] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::LinkedList;"] # [doc = ""] # [doc = " let list1 = LinkedList::from([1, 2, 3, 4]);"] # [doc = " let list2: LinkedList<_> = [1, 2, 3, 4].into();"] # [doc = " assert_eq!(list1, list2);"] # [doc = " ```"] fn from (arr : [T ; N]) -> Self { Self :: from_iter (arr) } }
};
}

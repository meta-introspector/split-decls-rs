// Generated macro for LinkedList (struct)
macro_rules! Depcrate_collections_linked_listLinkedList {
() => {
// Module: crate::collections::linked_list
// Provides: {"LinkedList"}
// Dependencies: {}
# [doc = " A doubly-linked list with owned nodes."] # [doc = ""] # [doc = " The `LinkedList` allows pushing and popping elements at either end"] # [doc = " in constant time."] # [doc = ""] # [doc = " A `LinkedList` with a known list of items can be initialized from an array:"] # [doc = " ```"] # [doc = " use std::collections::LinkedList;"] # [doc = ""] # [doc = " let list = LinkedList::from([1, 2, 3]);"] # [doc = " ```"] # [doc = ""] # [doc = " NOTE: It is almost always better to use [`Vec`] or [`VecDeque`] because"] # [doc = " array-based containers are generally faster,"] # [doc = " more memory efficient, and make better use of CPU cache."] # [doc = ""] # [doc = " [`Vec`]: crate::vec::Vec"] # [doc = " [`VecDeque`]: super::vec_deque::VecDeque"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "LinkedList")] # [rustc_insignificant_dtor] pub struct LinkedList < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { head : Option < NonNull < Node < T > > > , tail : Option < NonNull < Node < T > > > , len : usize , alloc : A , marker : PhantomData < Box < Node < T > , A > > , }
};
}

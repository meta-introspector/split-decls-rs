// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_linked_listIter {
() => {
// Module: crate::collections::linked_list
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the elements of a `LinkedList`."] # [doc = ""] # [doc = " This `struct` is created by [`LinkedList::iter()`]. See its"] # [doc = " documentation for more."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Iter < 'a , T : 'a > { head : Option < NonNull < Node < T > > > , tail : Option < NonNull < Node < T > > > , len : usize , marker : PhantomData < & 'a Node < T > > , }
};
}

// Generated macro for IterMut (struct)
macro_rules! Depcrate_collections_linked_listIterMut {
() => {
// Module: crate::collections::linked_list
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the elements of a `LinkedList`."] # [doc = ""] # [doc = " This `struct` is created by [`LinkedList::iter_mut()`]. See its"] # [doc = " documentation for more."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct IterMut < 'a , T : 'a > { head : Option < NonNull < Node < T > > > , tail : Option < NonNull < Node < T > > > , len : usize , marker : PhantomData < & 'a mut Node < T > > , }
};
}

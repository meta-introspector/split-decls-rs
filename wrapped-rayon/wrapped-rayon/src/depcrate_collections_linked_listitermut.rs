// Generated macro for IterMut (struct)
macro_rules! Depcrate_collections_linked_listIterMut {
() => {
// Module: crate::collections::linked_list
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Parallel iterator over a mutable reference to a linked list"] # [derive (Debug)] pub struct IterMut < 'a , T > { inner : vec :: IntoIter < & 'a mut T > , }
};
}

// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_linked_listIter {
() => {
// Module: crate::collections::linked_list
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Parallel iterator over an immutable reference to a linked list"] # [derive (Debug)] pub struct Iter < 'a , T > { inner : vec :: IntoIter < & 'a T > , }
};
}

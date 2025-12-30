// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator that yields mutable references to entries in the list."] pub struct IterMut < 'a , T > { entries : * mut Vec < Entry < T > > , # [doc = " The index of the head of the unvisited portion of the list."] head : Option < NonMaxUsize > , # [doc = " Because [`IterMut::entries`] is a pointer, we need to have a phantom data here for the lifetime parameter."] phantom : PhantomData < & 'a mut Vec < Entry < T > > > , # [doc = " The number of entries that have not been visited."] remaining : usize , # [doc = " The index of the tail of the unvisited portion of the list."] tail : Option < NonMaxUsize > , }
};
}

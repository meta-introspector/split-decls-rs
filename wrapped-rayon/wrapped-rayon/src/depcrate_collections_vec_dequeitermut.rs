// Generated macro for IterMut (struct)
macro_rules! Depcrate_collections_vec_dequeIterMut {
() => {
// Module: crate::collections::vec_deque
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Parallel iterator over a mutable reference to a double-ended queue"] # [derive (Debug)] pub struct IterMut < 'a , T > { inner : Chain < slice :: IterMut < 'a , T > , slice :: IterMut < 'a , T > > , }
};
}

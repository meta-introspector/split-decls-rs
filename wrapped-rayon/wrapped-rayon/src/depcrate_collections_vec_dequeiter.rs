// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_vec_dequeIter {
() => {
// Module: crate::collections::vec_deque
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Parallel iterator over an immutable reference to a double-ended queue"] # [derive (Debug)] pub struct Iter < 'a , T > { inner : Chain < slice :: Iter < 'a , T > , slice :: Iter < 'a , T > > , }
};
}

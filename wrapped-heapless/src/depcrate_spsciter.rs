// Generated macro for Iter (struct)
macro_rules! Depcrate_spscIter {
() => {
// Module: crate::spsc
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the items of a queue."] pub struct Iter < 'a , T > { rb : & 'a QueueView < T > , index : usize , len : usize , }
};
}

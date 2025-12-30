// Generated macro for TryIter (struct)
macro_rules! DepcrateTryIter {
() => {
// Module: crate
// Provides: {"TryIter"}
// Dependencies: {}
# [doc = " An iterator that pops items from a [`ConcurrentQueue`]."] # [doc = ""] # [doc = " This iterator will never block; it will return `None` once the queue has"] # [doc = " been exhausted. Calling `next` after `None` may yield `Some(item)` if more items"] # [doc = " are pushed to the queue."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct TryIter < 'a , T > { queue : & 'a ConcurrentQueue < T > , }
};
}

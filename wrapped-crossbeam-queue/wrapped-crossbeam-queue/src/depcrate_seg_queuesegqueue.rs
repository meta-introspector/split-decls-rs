// Generated macro for SegQueue (struct)
macro_rules! Depcrate_seg_queueSegQueue {
() => {
// Module: crate::seg_queue
// Provides: {"SegQueue"}
// Dependencies: {}
# [doc = " An unbounded multi-producer multi-consumer queue."] # [doc = ""] # [doc = " This queue is implemented as a linked list of segments, where each segment is a small buffer"] # [doc = " that can hold a handful of elements. There is no limit to how many elements can be in the queue"] # [doc = " at a time. However, since segments need to be dynamically allocated as elements get pushed,"] # [doc = " this queue is somewhat slower than [`ArrayQueue`]."] # [doc = ""] # [doc = " [`ArrayQueue`]: super::ArrayQueue"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_queue::SegQueue;"] # [doc = ""] # [doc = " let q = SegQueue::new();"] # [doc = ""] # [doc = " q.push('a');"] # [doc = " q.push('b');"] # [doc = ""] # [doc = " assert_eq!(q.pop(), Some('a'));"] # [doc = " assert_eq!(q.pop(), Some('b'));"] # [doc = " assert!(q.pop().is_none());"] # [doc = " ```"] pub struct SegQueue < T > { # [doc = " The head of the queue."] head : CachePadded < Position < T > > , # [doc = " The tail of the queue."] tail : CachePadded < Position < T > > , # [doc = " Indicates that dropping a `SegQueue<T>` may drop values of type `T`."] _marker : PhantomData < T > , }
};
}

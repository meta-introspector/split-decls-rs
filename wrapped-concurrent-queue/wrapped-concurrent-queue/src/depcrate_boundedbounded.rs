// Generated macro for Bounded (struct)
macro_rules! Depcrate_boundedBounded {
() => {
// Module: crate::bounded
// Provides: {"Bounded"}
// Dependencies: {}
# [doc = " A bounded queue."] pub struct Bounded < T > { # [doc = " The head of the queue."] # [doc = ""] # [doc = " This value is a \"stamp\" consisting of an index into the buffer, a mark bit, and a lap, but"] # [doc = " packed into a single `usize`. The lower bits represent the index, while the upper bits"] # [doc = " represent the lap. The mark bit in the head is always zero."] # [doc = ""] # [doc = " Values are popped from the head of the queue."] head : CachePadded < AtomicUsize > , # [doc = " The tail of the queue."] # [doc = ""] # [doc = " This value is a \"stamp\" consisting of an index into the buffer, a mark bit, and a lap, but"] # [doc = " packed into a single `usize`. The lower bits represent the index, while the upper bits"] # [doc = " represent the lap. The mark bit indicates that the queue is closed."] # [doc = ""] # [doc = " Values are pushed into the tail of the queue."] tail : CachePadded < AtomicUsize > , # [doc = " The buffer holding slots."] buffer : Box < [Slot < T >] > , # [doc = " A stamp with the value of `{ lap: 1, mark: 0, index: 0 }`."] one_lap : usize , # [doc = " If this bit is set in the tail, that means the queue is closed."] mark_bit : usize , }
};
}

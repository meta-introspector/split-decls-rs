// Generated macro for Consumer (struct)
macro_rules! Depcrate_spscConsumer {
() => {
// Module: crate::spsc
// Provides: {"Consumer"}
// Dependencies: {}
# [doc = " A consumer; it can dequeue items from the queue."] # [doc = ""] # [doc = " **Note:** The consumer semantically owns the `head` pointer of the queue."] pub struct Consumer < 'a , T > { rb : & 'a QueueView < T > , }
};
}

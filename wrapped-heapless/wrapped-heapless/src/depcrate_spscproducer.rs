// Generated macro for Producer (struct)
macro_rules! Depcrate_spscProducer {
() => {
// Module: crate::spsc
// Provides: {"Producer"}
// Dependencies: {}
# [doc = " A producer; it can enqueue items into the queue."] # [doc = ""] # [doc = " **Note:** The producer semantically owns the `tail` pointer of the queue."] pub struct Producer < 'a , T > { rb : & 'a QueueView < T > , }
};
}

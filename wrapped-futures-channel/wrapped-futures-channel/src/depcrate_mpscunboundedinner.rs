// Generated macro for UnboundedInner (struct)
macro_rules! Depcrate_mpscUnboundedInner {
() => {
// Module: crate::mpsc
// Provides: {"UnboundedInner"}
// Dependencies: {}
struct UnboundedInner < T > { state : AtomicUsize , message_queue : Queue < T > , num_senders : AtomicUsize , recv_task : AtomicWaker , }
};
}

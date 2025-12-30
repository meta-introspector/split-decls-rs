// Generated macro for BoundedInner (struct)
macro_rules! Depcrate_mpscBoundedInner {
() => {
// Module: crate::mpsc
// Provides: {"BoundedInner"}
// Dependencies: {}
struct BoundedInner < T > { buffer : usize , state : AtomicUsize , message_queue : Queue < T > , parked_queue : Queue < Arc < Mutex < SenderTask > > > , num_senders : AtomicUsize , recv_task : AtomicWaker , }
};
}

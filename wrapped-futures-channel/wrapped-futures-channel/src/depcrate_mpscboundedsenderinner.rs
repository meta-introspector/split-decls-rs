// Generated macro for BoundedSenderInner (struct)
macro_rules! Depcrate_mpscBoundedSenderInner {
() => {
// Module: crate::mpsc
// Provides: {"BoundedSenderInner"}
// Dependencies: {}
struct BoundedSenderInner < T > { inner : Arc < BoundedInner < T > > , sender_task : Arc < Mutex < SenderTask > > , maybe_parked : bool , }
};
}

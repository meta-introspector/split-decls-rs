// Generated macro for Notifier (struct)
macro_rules! Depcrate_future_future_sharedNotifier {
() => {
// Module: crate::future::future::shared
// Provides: {"Notifier"}
// Dependencies: {}
struct Notifier { state : AtomicUsize , wakers : Mutex < Option < Slab < Option < Waker > > > > , }
};
}

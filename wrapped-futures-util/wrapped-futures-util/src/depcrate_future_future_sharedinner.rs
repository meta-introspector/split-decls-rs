// Generated macro for Inner (struct)
macro_rules! Depcrate_future_future_sharedInner {
() => {
// Module: crate::future::future::shared
// Provides: {"Inner"}
// Dependencies: {}
struct Inner < Fut : Future > { future_or_output : UnsafeCell < FutureOrOutput < Fut > > , notifier : Arc < Notifier > , }
};
}

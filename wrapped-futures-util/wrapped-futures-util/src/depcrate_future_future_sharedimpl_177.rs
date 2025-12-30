// Generated macro for impl_177 (impl)
macro_rules! Depcrate_future_future_sharedimpl_177 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_177"}
// Dependencies: {}
impl < Fut : Future > Shared < Fut > { pub (super) fn new (future : Fut) -> Self { let inner = Inner { future_or_output : UnsafeCell :: new (FutureOrOutput :: Future (future)) , notifier : Arc :: new (Notifier { state : AtomicUsize :: new (IDLE) , wakers : Mutex :: new (Some (Slab :: new ())) , }) , } ; Self { inner : Some (Arc :: new (inner)) , waker_key : NULL_WAKER_KEY } } }
};
}

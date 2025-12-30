// Generated macro for impl_721 (impl)
macro_rules! Depcrate_future_wait_untilimpl_721 {
() => {
// Module: crate::future::wait_until
// Provides: {"impl_721"}
// Dependencies: {}
impl < F , D > WaitUntil < F , D > { pub (super) fn new (future : F , deadline : D) -> Self { Self { future , deadline , state : State :: Started , } } }
};
}

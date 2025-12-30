// Generated macro for impl_880 (impl)
macro_rules! Depcrate_stream_wait_untilimpl_880 {
() => {
// Module: crate::stream::wait_until
// Provides: {"impl_880"}
// Dependencies: {}
impl < S , D > WaitUntil < S , D > { pub (crate) fn new (stream : S , deadline : D) -> Self { WaitUntil { stream , deadline , state : State :: Timer , } } }
};
}

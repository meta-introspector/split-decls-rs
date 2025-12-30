// Generated macro for impl_324 (impl)
macro_rules! Depcrate_fut_future_timeoutimpl_324 {
() => {
// Module: crate::fut::future::timeout
// Provides: {"impl_324"}
// Dependencies: {}
impl < F > Timeout < F > { pub (super) fn new (future : F , timeout : Duration) -> Self { Self { fut : future , timeout : sleep (timeout) , } } }
};
}

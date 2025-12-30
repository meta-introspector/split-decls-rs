// Generated macro for impl_110 (impl)
macro_rules! Depcrate_future_future_mapimpl_110 {
() => {
// Module: crate::future::future::map
// Provides: {"impl_110"}
// Dependencies: {}
impl < Fut , F > Map < Fut , F > { # [doc = " Creates a new Map."] pub (crate) fn new (future : Fut , f : F) -> Self { Self :: Incomplete { future , f } } }
};
}

// Generated macro for impl_459 (impl)
macro_rules! Depcrate_common_timerimpl_459 {
() => {
// Module: crate::common::timer
// Provides: {"impl_459"}
// Dependencies: {}
impl Timer { pub (crate) fn new < T > (inner : T) -> Self where T : hyper :: rt :: Timer + Send + Sync + 'static , { Self (Arc :: new (inner)) } }
};
}

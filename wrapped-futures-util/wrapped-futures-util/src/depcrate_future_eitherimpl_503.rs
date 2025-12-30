// Generated macro for impl_503 (impl)
macro_rules! Depcrate_future_eitherimpl_503 {
() => {
// Module: crate::future::either
// Provides: {"impl_503"}
// Dependencies: {}
impl < T > Either < T , T > { # [doc = " Extract the value of an either over two equivalent types."] pub fn into_inner (self) -> T { match self { Self :: Left (x) | Self :: Right (x) => x , } } }
};
}

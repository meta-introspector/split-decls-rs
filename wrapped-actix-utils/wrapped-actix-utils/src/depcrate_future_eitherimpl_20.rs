// Generated macro for impl_20 (impl)
macro_rules! Depcrate_future_eitherimpl_20 {
() => {
// Module: crate::future::either
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > Either < T , T > { # [doc = " Unwraps into inner value when left and right have a common type."] # [inline] pub fn into_inner (self) -> T { match self { Either :: Left { value } => value , Either :: Right { value } => value , } } }
};
}

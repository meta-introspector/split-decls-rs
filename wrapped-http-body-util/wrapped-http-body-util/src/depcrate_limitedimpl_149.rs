// Generated macro for impl_149 (impl)
macro_rules! Depcrate_limitedimpl_149 {
() => {
// Module: crate::limited
// Provides: {"impl_149"}
// Dependencies: {}
impl < B > Limited < B > { # [doc = " Create a new `Limited`."] pub fn new (inner : B , limit : usize) -> Self { Self { remaining : limit , inner , } } }
};
}

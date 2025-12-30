// Generated macro for impl_105 (impl)
macro_rules! Depcrate_eitherimpl_105 {
() => {
// Module: crate::either
// Provides: {"impl_105"}
// Dependencies: {}
impl < L > Either < L , L > { # [doc = " Convert [`Either`] into the inner type, if both `Left` and `Right` are"] # [doc = " of the same type."] pub fn into_inner (self) -> L { match self { Either :: Left (left) => left , Either :: Right (right) => right , } } }
};
}

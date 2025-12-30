// Generated macro for impl_253 (impl)
macro_rules! Depcrate_builder_resettableimpl_253 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_253"}
// Dependencies: {}
impl < T > Resettable < T > { pub (crate) fn into_option (self) -> Option < T > { match self { Self :: Value (t) => Some (t) , Self :: Reset => None , } } }
};
}

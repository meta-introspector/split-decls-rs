// Generated macro for impl_82 (impl)
macro_rules! Depcrate_bindingsimpl_82 {
() => {
// Module: crate::bindings
// Provides: {"impl_82"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for & IVectorView < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

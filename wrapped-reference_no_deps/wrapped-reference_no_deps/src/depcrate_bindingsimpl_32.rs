// Generated macro for impl_32 (impl)
macro_rules! Depcrate_bindingsimpl_32 {
() => {
// Module: crate::bindings
// Provides: {"impl_32"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType > IntoIterator for & IIterable < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

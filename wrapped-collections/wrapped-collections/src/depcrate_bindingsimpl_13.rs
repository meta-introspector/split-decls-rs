// Generated macro for impl_13 (impl)
macro_rules! Depcrate_bindingsimpl_13 {
() => {
// Module: crate::bindings
// Provides: {"impl_13"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType > IntoIterator for & IIterable < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

// Generated macro for impl_69 (impl)
macro_rules! Depcrate_bindingsimpl_69 {
() => {
// Module: crate::bindings
// Provides: {"impl_69"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for & IVector < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

// Generated macro for impl_60 (impl)
macro_rules! Depcrate_bindingsimpl_60 {
() => {
// Module: crate::bindings
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for & IVector < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

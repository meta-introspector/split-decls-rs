// Generated macro for impl_81 (impl)
macro_rules! Depcrate_bindingsimpl_81 {
() => {
// Module: crate::bindings
// Provides: {"impl_81"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for IVectorView < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
};
}

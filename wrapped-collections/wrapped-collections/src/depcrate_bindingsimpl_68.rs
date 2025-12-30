// Generated macro for impl_68 (impl)
macro_rules! Depcrate_bindingsimpl_68 {
() => {
// Module: crate::bindings
// Provides: {"impl_68"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for IVector < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
};
}

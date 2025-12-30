// Generated macro for impl_483 (impl)
macro_rules! Depcrate_interface_iterableimpl_483 {
() => {
// Module: crate::interface_iterable
// Provides: {"impl_483"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for IVector < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
};
}

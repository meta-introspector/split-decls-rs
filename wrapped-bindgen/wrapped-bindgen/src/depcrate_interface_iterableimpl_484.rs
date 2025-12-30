// Generated macro for impl_484 (impl)
macro_rules! Depcrate_interface_iterableimpl_484 {
() => {
// Module: crate::interface_iterable
// Provides: {"impl_484"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for & IVector < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

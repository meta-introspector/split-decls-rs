// Generated macro for impl_464 (impl)
macro_rules! Depcrate_interface_iterableimpl_464 {
() => {
// Module: crate::interface_iterable
// Provides: {"impl_464"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType > IntoIterator for & IIterable < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

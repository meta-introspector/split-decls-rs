// Generated macro for impl_463 (impl)
macro_rules! Depcrate_interface_iterableimpl_463 {
() => {
// Module: crate::interface_iterable
// Provides: {"impl_463"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType > IntoIterator for IIterable < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
};
}

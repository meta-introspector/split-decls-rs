// Generated macro for impl_63 (impl)
macro_rules! Depcrate_class_depimpl_63 {
() => {
// Module: crate::class_dep
// Provides: {"impl_63"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType > IntoIterator for & IIterable < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

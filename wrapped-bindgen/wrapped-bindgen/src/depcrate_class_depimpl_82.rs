// Generated macro for impl_82 (impl)
macro_rules! Depcrate_class_depimpl_82 {
() => {
// Module: crate::class_dep
// Provides: {"impl_82"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for IVectorView < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (& self) } }
};
}

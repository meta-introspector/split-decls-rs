// Generated macro for impl_83 (impl)
macro_rules! Depcrate_class_depimpl_83 {
() => {
// Module: crate::class_dep
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > IntoIterator for & IVectorView < T > { type Item = T ; type IntoIter = IIterator < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . First () . unwrap () } }
};
}

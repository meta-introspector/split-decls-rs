// Generated macro for impl_34 (impl)
macro_rules! Depcrate_arrayimpl_34 {
() => {
// Module: crate::array
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , T : Type > IntoIterator for & 'a CFMutableArray < T > { type Item = CFRetained < T > ; type IntoIter = CFArrayIter < 'a , T > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}

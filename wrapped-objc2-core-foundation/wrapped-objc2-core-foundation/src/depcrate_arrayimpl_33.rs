// Generated macro for impl_33 (impl)
macro_rules! Depcrate_arrayimpl_33 {
() => {
// Module: crate::array
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , T : Type > IntoIterator for & 'a CFArray < T > { type Item = CFRetained < T > ; type IntoIter = CFArrayIter < 'a , T > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}

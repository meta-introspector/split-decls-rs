// Generated macro for impl_27 (impl)
macro_rules! Depcrate_arrayimpl_27 {
() => {
// Module: crate::array
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , T : FromVoid > IntoIterator for & 'a CFArray < T > { type Item = ItemRef < 'a , T > ; type IntoIter = CFArrayIterator < 'a , T > ; fn into_iter (self) -> CFArrayIterator < 'a , T > { self . iter () } }
};
}

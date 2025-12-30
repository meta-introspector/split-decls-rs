// Generated macro for impl_838 (impl)
macro_rules! Depcrate_vtabimpl_838 {
() => {
// Module: crate::vtab
// Provides: {"impl_838"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Values < 'a > { type IntoIter = ValueIter < 'a > ; type Item = ValueRef < 'a > ; # [inline] fn into_iter (self) -> ValueIter < 'a > { self . iter () } }
};
}

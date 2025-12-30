// Generated macro for impl_8 (impl)
macro_rules! Depcrate_argsimpl_8 {
() => {
// Module: crate::args
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'args > IntoIterator for FluentArgs < 'args > { type Item = (Cow < 'args , str > , FluentValue < 'args >) ; type IntoIter = std :: vec :: IntoIter < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . into_iter () } }
};
}

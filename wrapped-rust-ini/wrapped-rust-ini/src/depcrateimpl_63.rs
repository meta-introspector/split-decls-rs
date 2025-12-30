// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Ini { type IntoIter = SectionIterMut < 'a > ; type Item = (Option < & 'a str > , & 'a mut Properties) ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}

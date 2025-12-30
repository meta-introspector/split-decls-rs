// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Properties { type IntoIter = PropertyIterMut < 'a > ; type Item = (& 'a str , & 'a mut String) ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}

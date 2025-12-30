// Generated macro for impl_58 (impl)
macro_rules! Depcrate_header_mapimpl_58 {
() => {
// Module: crate::header::map
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a mut HeaderMap < T > { type Item = (& 'a HeaderName , & 'a mut T) ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}

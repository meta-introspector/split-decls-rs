// Generated macro for impl_57 (impl)
macro_rules! Depcrate_header_mapimpl_57 {
() => {
// Module: crate::header::map
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a HeaderMap < T > { type Item = (& 'a HeaderName , & 'a T) ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}

// Generated macro for impl_63 (impl)
macro_rules! Depcrate_header_mapimpl_63 {
() => {
// Module: crate::header::map
// Provides: {"impl_63"}
// Dependencies: {}
impl < T > Extend < (HeaderName , T) > for HeaderMap < T > { fn extend < I : IntoIterator < Item = (HeaderName , T) > > (& mut self , iter : I) { let iter = iter . into_iter () ; let reserve = if self . is_empty () { iter . size_hint () . 0 } else { (iter . size_hint () . 0 + 1) / 2 } ; self . reserve (reserve) ; for (k , v) in iter { self . append (k , v) ; } } }
};
}

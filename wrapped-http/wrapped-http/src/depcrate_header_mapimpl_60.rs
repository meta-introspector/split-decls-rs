// Generated macro for impl_60 (impl)
macro_rules! Depcrate_header_mapimpl_60 {
() => {
// Module: crate::header::map
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > FromIterator < (HeaderName , T) > for HeaderMap < T > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (HeaderName , T) > , { let mut map = HeaderMap :: default () ; map . extend (iter) ; map } }
};
}

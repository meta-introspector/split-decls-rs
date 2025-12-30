// Generated macro for impl_440 (impl)
macro_rules! Depcrate_filtersimpl_440 {
() => {
// Module: crate::filters
// Provides: {"impl_440"}
// Dependencies: {}
impl < 'a > FromIterator < (& 'a str , & 'a str) > for Filters { fn from_iter < I : IntoIterator < Item = (& 'a str , & 'a str) > > (iter : I) -> Self { let mut rv = Filters :: default () ; for (regex , replacement) in iter { rv . add (regex , replacement) ; } rv } }
};
}

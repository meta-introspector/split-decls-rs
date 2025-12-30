// Generated macro for impl_439 (impl)
macro_rules! Depcrate_filtersimpl_439 {
() => {
// Module: crate::filters
// Provides: {"impl_439"}
// Dependencies: {}
impl < 'a , I > From < I > for Filters where I : IntoIterator < Item = (& 'a str , & 'a str) > , { fn from (value : I) -> Self { Self :: from_iter (value) } }
};
}

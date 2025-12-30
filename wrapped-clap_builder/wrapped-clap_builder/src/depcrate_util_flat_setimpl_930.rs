// Generated macro for impl_930 (impl)
macro_rules! Depcrate_util_flat_setimpl_930 {
() => {
// Module: crate::util::flat_set
// Provides: {"impl_930"}
// Dependencies: {}
impl < T : PartialEq + Eq > FromIterator < T > for FlatSet < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let mut set = Self :: new () ; for value in iter { set . insert (value) ; } set } }
};
}

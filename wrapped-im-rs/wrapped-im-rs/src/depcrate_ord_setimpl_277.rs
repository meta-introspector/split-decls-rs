// Generated macro for impl_277 (impl)
macro_rules! Depcrate_ord_setimpl_277 {
() => {
// Module: crate::ord::set
// Provides: {"impl_277"}
// Dependencies: {}
impl < A , R > FromIterator < R > for OrdSet < A > where A : Ord + Clone + From < R > , { fn from_iter < T > (i : T) -> Self where T : IntoIterator < Item = R > , { let mut out = Self :: new () ; for item in i { out . insert (From :: from (item)) ; } out } }
};
}

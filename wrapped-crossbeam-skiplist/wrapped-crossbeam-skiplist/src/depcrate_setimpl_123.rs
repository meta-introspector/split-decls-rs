// Generated macro for impl_123 (impl)
macro_rules! Depcrate_setimpl_123 {
() => {
// Module: crate::set
// Provides: {"impl_123"}
// Dependencies: {}
impl < T > FromIterator < T > for SkipSet < T > where T : Ord , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { let s = Self :: new () ; for t in iter { s . get_or_insert (t) ; } s } }
};
}

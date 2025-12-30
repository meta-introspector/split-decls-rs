// Generated macro for impl_89 (impl)
macro_rules! Depcrate_mapimpl_89 {
() => {
// Module: crate::map
// Provides: {"impl_89"}
// Dependencies: {}
impl < K , V > FromIterator < (K , V) > for SkipMap < K , V > where K : Ord , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let s = Self :: new () ; for (k , v) in iter { s . get_or_insert (k , v) ; } s } }
};
}

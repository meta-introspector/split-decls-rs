// Generated macro for impl_213 (impl)
macro_rules! Depcrate_linear_mapimpl_213 {
() => {
// Module: crate::linear_map
// Provides: {"impl_213"}
// Dependencies: {}
impl < K , V , const N : usize > FromIterator < (K , V) > for LinearMap < K , V , N > where K : Eq , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut out = Self :: new () ; out . buffer . extend (iter) ; out } }
};
}

// Generated macro for impl_127 (impl)
macro_rules! Depcrate_index_mapimpl_127 {
() => {
// Module: crate::index_map
// Provides: {"impl_127"}
// Dependencies: {}
impl < K , V , S , const N : usize > IndexMap < K , V , BuildHasherDefault < S > , N > { # [doc = " Creates an empty `IndexMap`."] pub const fn new () -> Self { const { assert ! (N > 1) ; assert ! (N . is_power_of_two ()) ; } Self { build_hasher : BuildHasherDefault :: new () , core : CoreMap :: new () , } } }
};
}

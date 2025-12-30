// Generated macro for impl_134 (impl)
macro_rules! Depcrate_index_mapimpl_134 {
() => {
// Module: crate::index_map
// Provides: {"impl_134"}
// Dependencies: {}
impl < K , V , S , const N : usize > Default for IndexMap < K , V , S , N > where S : Default , { fn default () -> Self { const { assert ! (N > 1) ; assert ! (N . is_power_of_two ()) ; } Self { build_hasher : < _ > :: default () , core : CoreMap :: new () , } } }
};
}

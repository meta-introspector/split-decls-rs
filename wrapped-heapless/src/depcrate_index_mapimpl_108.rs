// Generated macro for impl_108 (impl)
macro_rules! Depcrate_index_mapimpl_108 {
() => {
// Module: crate::index_map
// Provides: {"impl_108"}
// Dependencies: {}
impl HashValue { fn desired_pos (& self , mask : usize) -> usize { usize :: from (self . 0) & mask } fn probe_distance (& self , mask : usize , current : usize) -> usize { current . wrapping_sub (self . desired_pos (mask)) & mask } }
};
}

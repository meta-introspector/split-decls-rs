// Generated macro for impl_645 (impl)
macro_rules! Depcrate_config_transportimpl_645 {
() => {
// Module: crate::config::transport
// Provides: {"impl_645"}
// Dependencies: {}
impl Default for MtuDiscoveryConfig { fn default () -> Self { Self { interval : Duration :: from_secs (600) , upper_bound : 1452 , black_hole_cooldown : Duration :: from_secs (60) , minimum_change : 20 , } } }
};
}

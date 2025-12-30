// Generated macro for impl_663 (impl)
macro_rules! Depcrate_configimpl_663 {
() => {
// Module: crate::config
// Provides: {"impl_663"}
// Dependencies: {}
impl Default for ValidationTokenConfig { fn default () -> Self { # [cfg (feature = "bloom")] let log = Arc :: new (BloomTokenLog :: default ()) ; # [cfg (not (feature = "bloom"))] let log = Arc :: new (NoneTokenLog) ; Self { lifetime : Duration :: from_secs (2 * 7 * 24 * 60 * 60) , log , sent : if cfg ! (feature = "bloom") { 2 } else { 0 } , } } }
};
}

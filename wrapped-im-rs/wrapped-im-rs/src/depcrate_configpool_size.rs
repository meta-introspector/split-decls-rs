// Generated macro for POOL_SIZE (const)
macro_rules! Depcrate_configPOOL_SIZE {
() => {
// Module: crate::config
// Provides: {"POOL_SIZE"}
// Dependencies: {}
# [doc = " The size of per-instance memory pools if the `pool` feature is enabled."] # [doc = " This is set to 0, meaning you have to opt in to using a pool by constructing"] # [doc = " with eg. `Vector::with_pool(pool)` even if the `pool` feature is enabled."] pub (crate) const POOL_SIZE : usize = 0 ;
};
}

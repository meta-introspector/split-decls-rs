// Generated macro for quiche_h3_config_set_qpack_max_table_capacity (function)
macro_rules! Depcrate_h3_ffiquiche_h3_config_set_qpack_max_table_capacity {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_config_set_qpack_max_table_capacity"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_config_set_qpack_max_table_capacity (config : & mut h3 :: Config , v : u64 ,) { config . set_qpack_max_table_capacity (v) ; }
};
}

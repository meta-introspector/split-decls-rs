// Generated macro for macro_2 (macro)
macro_rules! Depcratemacro_2 {
() => {
// Module: crate
// Provides: {"macro_2"}
// Dependencies: {}
# [rustfmt :: skip] # [cfg (any (all (feature = "release_max_level_off" , feature = "release_max_level_error") , all (feature = "release_max_level_off" , feature = "release_max_level_warn") , all (feature = "release_max_level_off" , feature = "release_max_level_info") , all (feature = "release_max_level_off" , feature = "release_max_level_debug") , all (feature = "release_max_level_off" , feature = "release_max_level_trace") , all (feature = "release_max_level_error" , feature = "release_max_level_warn") , all (feature = "release_max_level_error" , feature = "release_max_level_info") , all (feature = "release_max_level_error" , feature = "release_max_level_debug") , all (feature = "release_max_level_error" , feature = "release_max_level_trace") , all (feature = "release_max_level_warn" , feature = "release_max_level_info") , all (feature = "release_max_level_warn" , feature = "release_max_level_debug") , all (feature = "release_max_level_warn" , feature = "release_max_level_trace") , all (feature = "release_max_level_info" , feature = "release_max_level_debug") , all (feature = "release_max_level_info" , feature = "release_max_level_trace") , all (feature = "release_max_level_debug" , feature = "release_max_level_trace") ,))] compile_error ! ("multiple release_max_level_* features set") ;
};
}

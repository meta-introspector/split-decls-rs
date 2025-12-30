// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (any (all (feature = "max_level_off" , feature = "max_level_error") , all (feature = "max_level_off" , feature = "max_level_warn") , all (feature = "max_level_off" , feature = "max_level_info") , all (feature = "max_level_off" , feature = "max_level_debug") , all (feature = "max_level_off" , feature = "max_level_trace") , all (feature = "max_level_error" , feature = "max_level_warn") , all (feature = "max_level_error" , feature = "max_level_info") , all (feature = "max_level_error" , feature = "max_level_debug") , all (feature = "max_level_error" , feature = "max_level_trace") , all (feature = "max_level_warn" , feature = "max_level_info") , all (feature = "max_level_warn" , feature = "max_level_debug") , all (feature = "max_level_warn" , feature = "max_level_trace") , all (feature = "max_level_info" , feature = "max_level_debug") , all (feature = "max_level_info" , feature = "max_level_trace") , all (feature = "max_level_debug" , feature = "max_level_trace") ,))] compile_error ! ("multiple max_level_* features set") ;
};
}

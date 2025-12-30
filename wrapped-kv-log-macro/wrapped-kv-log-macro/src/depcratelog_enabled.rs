// Generated macro for log_enabled (macro)
macro_rules! Depcratelog_enabled {
() => {
// Module: crate
// Provides: {"log_enabled"}
// Dependencies: {}
# [doc = " Determines if a message logged at the specified level in that module will"] # [doc = " be logged."] # [macro_export (local_inner_macros)] macro_rules ! log_enabled { (target : $ target : expr , $ lvl : expr) => { { let lvl = $ lvl ; lvl <= $ crate :: STATIC_MAX_LEVEL && lvl <= $ crate :: max_level () && $ crate :: __private_api_enabled (lvl , $ target) } } ; ($ lvl : expr) => { log_enabled ! (target : __log_module_path ! () , $ lvl) } ; }
};
}

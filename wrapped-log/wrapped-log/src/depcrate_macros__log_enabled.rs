// Generated macro for __log_enabled (macro)
macro_rules! Depcrate_macros__log_enabled {
() => {
// Module: crate::macros
// Provides: {"__log_enabled"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __log_enabled { (logger : $ logger : expr , target : $ target : expr , $ lvl : expr) => { { let lvl = $ lvl ; lvl <= $ crate :: STATIC_MAX_LEVEL && lvl <= $ crate :: max_level () && $ crate :: __private_api :: enabled ($ logger , lvl , $ target) } } ; }
};
}

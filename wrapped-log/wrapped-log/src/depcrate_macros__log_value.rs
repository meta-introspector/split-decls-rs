// Generated macro for __log_value (macro)
macro_rules! Depcrate_macros__log_value {
() => {
// Module: crate::macros
// Provides: {"__log_value"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (not (feature = "kv"))] macro_rules ! __log_value { ($ ($ args : tt) *) => { compile_error ! ("key value support requires the `kv` feature of `log`") } ; }
};
}

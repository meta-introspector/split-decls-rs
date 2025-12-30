// Generated macro for __log_key (macro)
macro_rules! Depcrate_macros__log_key {
() => {
// Module: crate::macros
// Provides: {"__log_key"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (not (feature = "kv"))] macro_rules ! __log_key { ($ ($ args : tt) *) => { compile_error ! ("key value support requires the `kv` feature of `log`") } ; }
};
}

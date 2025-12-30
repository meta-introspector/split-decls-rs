// Generated macro for __log_value_serde (macro)
macro_rules! Depcrate_macros__log_value_serde {
() => {
// Module: crate::macros
// Provides: {"__log_value_serde"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (not (feature = "kv_serde"))] macro_rules ! __log_value_serde { ($ args : expr) => { compile_error ! ("capturing values as `serde::Serialize` requites the `kv_serde` feature of `log`") } ; }
};
}

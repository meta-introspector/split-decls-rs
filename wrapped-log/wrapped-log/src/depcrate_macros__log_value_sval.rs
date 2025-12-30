// Generated macro for __log_value_sval (macro)
macro_rules! Depcrate_macros__log_value_sval {
() => {
// Module: crate::macros
// Provides: {"__log_value_sval"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (not (feature = "kv_sval"))] macro_rules ! __log_value_sval { ($ args : expr) => { compile_error ! ("capturing values as `sval::Value` requites the `kv_sval` feature of `log`") } ; }
};
}

// Generated macro for __log_value_error (macro)
macro_rules! Depcrate_macros__log_value_error {
() => {
// Module: crate::macros
// Provides: {"__log_value_error"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (not (feature = "kv_std"))] macro_rules ! __log_value_error { ($ args : expr) => { compile_error ! ("capturing values as `std::error::Error` requites the `kv_std` feature of `log`") } ; }
};
}

// Generated macro for rustls_log_level_str (function)
macro_rules! Depcrate_logrustls_log_level_str {
() => {
// Module: crate::log
// Provides: {"rustls_log_level_str"}
// Dependencies: {}
# [doc = " Return a rustls_str containing the stringified version of a log level."] # [no_mangle] pub extern "C" fn rustls_log_level_str (level : rustls_log_level) -> rustls_str < 'static > { let s = match level { 1 => Level :: Error . as_str () , 2 => Level :: Warn . as_str () , 3 => Level :: Info . as_str () , 4 => Level :: Debug . as_str () , 5 => Level :: Trace . as_str () , _ => "INVALID" , } ; rustls_str :: from_str_unchecked (s) }
};
}

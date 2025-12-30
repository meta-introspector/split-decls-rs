// Generated macro for rustls_log_params (struct)
macro_rules! Depcrate_logrustls_log_params {
() => {
// Module: crate::log
// Provides: {"rustls_log_params"}
// Dependencies: {}
# [doc = " Parameter structure passed to a `rustls_log_callback`."] # [repr (C)] pub struct rustls_log_params < 'a > { # [doc = " The log level the message was logged at."] pub level : rustls_log_level , # [doc = " The message that was logged."] pub message : rustls_str < 'a > , }
};
}

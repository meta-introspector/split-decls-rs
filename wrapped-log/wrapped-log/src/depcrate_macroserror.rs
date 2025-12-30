// Generated macro for error (macro)
macro_rules! Depcrate_macroserror {
() => {
// Module: crate::macros
// Provides: {"error"}
// Dependencies: {}
# [doc = " Logs a message at the error level."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use log::error;"] # [doc = ""] # [doc = " # let my_logger = log::__private_api::GlobalLogger;"] # [doc = " let (err_info, port) = (\"No connection\", 22);"] # [doc = ""] # [doc = " error!(\"Error: {err_info} on port {port}\");"] # [doc = " error!(target: \"app_events\", \"App Error: {err_info}, Port: {port}\");"] # [doc = " error!(logger: my_logger, \"App Error: {err_info}, Port: {port}\");"] # [doc = " ```"] # [macro_export] # [clippy :: format_args] macro_rules ! error { (logger : $ logger : expr , target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , target : $ target , $ crate :: Level :: Error , $ ($ arg) +) }) ; (logger : $ logger : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , $ crate :: Level :: Error , $ ($ arg) +) }) ; (target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (target : $ target , $ crate :: Level :: Error , $ ($ arg) +) }) ; ($ ($ arg : tt) +) => ($ crate :: log ! ($ crate :: Level :: Error , $ ($ arg) +)) }
};
}

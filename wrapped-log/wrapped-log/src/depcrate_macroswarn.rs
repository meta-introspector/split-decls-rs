// Generated macro for warn (macro)
macro_rules! Depcrate_macroswarn {
() => {
// Module: crate::macros
// Provides: {"warn"}
// Dependencies: {}
# [doc = " Logs a message at the warn level."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use log::warn;"] # [doc = ""] # [doc = " # let my_logger = log::__private_api::GlobalLogger;"] # [doc = " let warn_description = \"Invalid Input\";"] # [doc = ""] # [doc = " warn!(\"Warning! {warn_description}!\");"] # [doc = " warn!(target: \"input_events\", \"App received warning: {warn_description}\");"] # [doc = " warn!(logger: my_logger, \"App received warning: {warn_description}\");"] # [doc = " ```"] # [macro_export] # [clippy :: format_args] macro_rules ! warn { (logger : $ logger : expr , target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , target : $ target , $ crate :: Level :: Warn , $ ($ arg) +) }) ; (logger : $ logger : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , $ crate :: Level :: Warn , $ ($ arg) +) }) ; (target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (target : $ target , $ crate :: Level :: Warn , $ ($ arg) +) }) ; ($ ($ arg : tt) +) => ($ crate :: log ! ($ crate :: Level :: Warn , $ ($ arg) +)) }
};
}

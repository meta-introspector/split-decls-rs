// Generated macro for debug (macro)
macro_rules! Depcrate_macrosdebug {
() => {
// Module: crate::macros
// Provides: {"debug"}
// Dependencies: {}
# [doc = " Logs a message at the debug level."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use log::debug;"] # [doc = ""] # [doc = " # let my_logger = log::__private_api::GlobalLogger;"] # [doc = " # struct Position { x: f32, y: f32 }"] # [doc = " let pos = Position { x: 3.234, y: -1.223 };"] # [doc = ""] # [doc = " debug!(\"New position: x: {}, y: {}\", pos.x, pos.y);"] # [doc = " debug!(target: \"app_events\", \"New position: x: {}, y: {}\", pos.x, pos.y);"] # [doc = " debug!(logger: my_logger, \"New position: x: {}, y: {}\", pos.x, pos.y);"] # [doc = " ```"] # [macro_export] # [clippy :: format_args] macro_rules ! debug { (logger : $ logger : expr , target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , target : $ target , $ crate :: Level :: Debug , $ ($ arg) +) }) ; (logger : $ logger : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , $ crate :: Level :: Debug , $ ($ arg) +) }) ; (target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (target : $ target , $ crate :: Level :: Debug , $ ($ arg) +) }) ; ($ ($ arg : tt) +) => ($ crate :: log ! ($ crate :: Level :: Debug , $ ($ arg) +)) }
};
}

// Generated macro for trace (macro)
macro_rules! Depcrate_macrostrace {
() => {
// Module: crate::macros
// Provides: {"trace"}
// Dependencies: {}
# [doc = " Logs a message at the trace level."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use log::trace;"] # [doc = ""] # [doc = " # let my_logger = log::__private_api::GlobalLogger;"] # [doc = " # struct Position { x: f32, y: f32 }"] # [doc = " let pos = Position { x: 3.234, y: -1.223 };"] # [doc = ""] # [doc = " trace!(\"Position is: x: {}, y: {}\", pos.x, pos.y);"] # [doc = " trace!(target: \"app_events\", \"x is {} and y is {}\","] # [doc = "        if pos.x >= 0.0 { \"positive\" } else { \"negative\" },"] # [doc = "        if pos.y >= 0.0 { \"positive\" } else { \"negative\" });"] # [doc = " trace!(logger: my_logger, \"x is {} and y is {}\","] # [doc = "        if pos.x >= 0.0 { \"positive\" } else { \"negative\" },"] # [doc = "        if pos.y >= 0.0 { \"positive\" } else { \"negative\" });"] # [doc = " ```"] # [macro_export] # [clippy :: format_args] macro_rules ! trace { (logger : $ logger : expr , target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , target : $ target , $ crate :: Level :: Trace , $ ($ arg) +) }) ; (logger : $ logger : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , $ crate :: Level :: Trace , $ ($ arg) +) }) ; (target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (target : $ target , $ crate :: Level :: Trace , $ ($ arg) +) }) ; ($ ($ arg : tt) +) => ($ crate :: log ! ($ crate :: Level :: Trace , $ ($ arg) +)) }
};
}

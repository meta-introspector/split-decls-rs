// Generated macro for info (macro)
macro_rules! Depcrate_macrosinfo {
() => {
// Module: crate::macros
// Provides: {"info"}
// Dependencies: {}
# [doc = " Logs a message at the info level."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use log::info;"] # [doc = ""] # [doc = " # let my_logger = log::__private_api::GlobalLogger;"] # [doc = " # struct Connection { port: u32, speed: f32 }"] # [doc = " let conn_info = Connection { port: 40, speed: 3.20 };"] # [doc = ""] # [doc = " info!(\"Connected to port {} at {} Mb/s\", conn_info.port, conn_info.speed);"] # [doc = " info!("] # [doc = "     target: \"connection_events\","] # [doc = "     \"Successful connection, port: {}, speed: {}\","] # [doc = "     conn_info.port, conn_info.speed"] # [doc = " );"] # [doc = " info!("] # [doc = "     logger: my_logger,"] # [doc = "     \"Successful connection, port: {}, speed: {}\","] # [doc = "     conn_info.port, conn_info.speed"] # [doc = " );"] # [doc = " ```"] # [macro_export] # [clippy :: format_args] macro_rules ! info { (logger : $ logger : expr , target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , target : $ target , $ crate :: Level :: Info , $ ($ arg) +) }) ; (logger : $ logger : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (logger : $ crate :: __log_logger ! ($ logger) , $ crate :: Level :: Info , $ ($ arg) +) }) ; (target : $ target : expr , $ ($ arg : tt) +) => ({ $ crate :: log ! (target : $ target , $ crate :: Level :: Info , $ ($ arg) +) }) ; ($ ($ arg : tt) +) => ($ crate :: log ! ($ crate :: Level :: Info , $ ($ arg) +)) }
};
}

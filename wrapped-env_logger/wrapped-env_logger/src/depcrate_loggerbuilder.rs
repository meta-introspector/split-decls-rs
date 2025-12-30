// Generated macro for Builder (struct)
macro_rules! Depcrate_loggerBuilder {
() => {
// Module: crate::logger
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " `Builder` acts as builder for initializing a `Logger`."] # [doc = ""] # [doc = " It can be used to customize the log format, change the environment variable used"] # [doc = " to provide the logging directives and also set the default log level filter."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::io::Write;"] # [doc = " use env_logger::Builder;"] # [doc = " use log::{LevelFilter, error, info};"] # [doc = ""] # [doc = " let mut builder = Builder::from_default_env();"] # [doc = ""] # [doc = " builder"] # [doc = "     .format(|buf, record| writeln!(buf, \"{} - {}\", record.level(), record.args()))"] # [doc = "     .filter(None, LevelFilter::Info)"] # [doc = "     .init();"] # [doc = ""] # [doc = " error!(\"error message\");"] # [doc = " info!(\"info message\");"] # [doc = " ```"] # [derive (Default)] pub struct Builder { filter : env_filter :: Builder , writer : writer :: Builder , format : fmt :: Builder , built : bool , }
};
}

// Generated macro for Metadata (struct)
macro_rules! DepcrateMetadata {
() => {
// Module: crate
// Provides: {"Metadata"}
// Dependencies: {}
# [doc = " Metadata about a log message."] # [doc = ""] # [doc = " # Use"] # [doc = ""] # [doc = " `Metadata` structs are created when users of the library use"] # [doc = " logging macros."] # [doc = ""] # [doc = " They are consumed by implementations of the `Log` trait in the"] # [doc = " `enabled` method."] # [doc = ""] # [doc = " `Record`s use `Metadata` to determine the log message's severity"] # [doc = " and target."] # [doc = ""] # [doc = " Users should use the `log_enabled!` macro in their code to avoid"] # [doc = " constructing expensive log messages."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use log::{Record, Level, Metadata};"] # [doc = ""] # [doc = " struct MyLogger;"] # [doc = ""] # [doc = " impl log::Log for MyLogger {"] # [doc = "     fn enabled(&self, metadata: &Metadata) -> bool {"] # [doc = "         metadata.level() <= Level::Info"] # [doc = "     }"] # [doc = ""] # [doc = "     fn log(&self, record: &Record) {"] # [doc = "         if self.enabled(record.metadata()) {"] # [doc = "             println!(\"{} - {}\", record.level(), record.args());"] # [doc = "         }"] # [doc = "     }"] # [doc = "     fn flush(&self) {}"] # [doc = " }"] # [doc = ""] # [doc = " # fn main(){}"] # [doc = " ```"] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Metadata < 'a > { level : Level , target : & 'a str , }
};
}

// Generated macro for Formatter (struct)
macro_rules! Depcrate_fmtFormatter {
() => {
// Module: crate::fmt
// Provides: {"Formatter"}
// Dependencies: {}
# [doc = " A formatter to write logs into."] # [doc = ""] # [doc = " `Formatter` implements the standard [`Write`] trait for writing log records."] # [doc = " It also supports terminal styling using ANSI escape codes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Use the [`writeln`] macro to format a log record."] # [doc = " An instance of a `Formatter` is passed to an `env_logger` format as `buf`:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::Write;"] # [doc = ""] # [doc = " let mut builder = env_logger::Builder::new();"] # [doc = ""] # [doc = " builder.format(|buf, record| writeln!(buf, \"{}: {}\", record.level(), record.args()));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Write`]: std::io::Write"] # [doc = " [`writeln`]: std::writeln"] pub struct Formatter { buf : Rc < RefCell < Buffer > > , write_style : WriteStyle , }
};
}

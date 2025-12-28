macro_rules! deps {
    () => {
        WriteStyle!();
        Buffer!();
    };
}

macro_rules! Formatter {
    () => {
        deps!();
        # [doc = " A formatter to write logs into."] # [doc = ""] # [doc = " `Formatter` implements the standard [`Write`] trait for writing log records."] # [doc = " It also supports terminal styling using ANSI escape codes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Use the [`writeln`] macro to format a log record."] # [doc = " An instance of a `Formatter` is passed to an `env_logger` format as `buf`:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::Write;"] # [doc = ""] # [doc = " let mut builder = env_logger::Builder::new();"] # [doc = ""] # [doc = " builder.format(|buf, record| writeln!(buf, \"{}: {}\", record.level(), record.args()));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Write`]: std::io::Write"] # [doc = " [`writeln`]: std::writeln"] pub struct Formatter { buf : Rc < RefCell < Buffer > > , write_style : WriteStyle , }
    };
}

Formatter!()
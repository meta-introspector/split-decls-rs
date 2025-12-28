macro_rules! deps {
    () => {
        TimestampPrecision!();
        Timestamp!();
        Formatter!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Formatter { # [doc = " Get a [`Timestamp`] for the current date and time in UTC."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Include the current timestamp with the log record:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::Write;"] # [doc = ""] # [doc = " let mut builder = env_logger::Builder::new();"] # [doc = ""] # [doc = " builder.format(|buf, record| {"] # [doc = "     let ts = buf.timestamp();"] # [doc = ""] # [doc = "     writeln!(buf, \"{}: {}: {}\", ts, record.level(), record.args())"] # [doc = " });"] # [doc = " ```"] pub fn timestamp (& self) -> Timestamp { Timestamp { time : SystemTime :: now () , precision : TimestampPrecision :: Seconds , } } # [doc = " Get a [`Timestamp`] for the current date and time in UTC with full"] # [doc = " second precision."] pub fn timestamp_seconds (& self) -> Timestamp { Timestamp { time : SystemTime :: now () , precision : TimestampPrecision :: Seconds , } } # [doc = " Get a [`Timestamp`] for the current date and time in UTC with"] # [doc = " millisecond precision."] pub fn timestamp_millis (& self) -> Timestamp { Timestamp { time : SystemTime :: now () , precision : TimestampPrecision :: Millis , } } # [doc = " Get a [`Timestamp`] for the current date and time in UTC with"] # [doc = " microsecond precision."] pub fn timestamp_micros (& self) -> Timestamp { Timestamp { time : SystemTime :: now () , precision : TimestampPrecision :: Micros , } } # [doc = " Get a [`Timestamp`] for the current date and time in UTC with"] # [doc = " nanosecond precision."] pub fn timestamp_nanos (& self) -> Timestamp { Timestamp { time : SystemTime :: now () , precision : TimestampPrecision :: Nanos , } } }
    };
}

impl_46!();
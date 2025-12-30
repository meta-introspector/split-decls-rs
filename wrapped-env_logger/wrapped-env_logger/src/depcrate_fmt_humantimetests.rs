// Generated macro for tests (module)
macro_rules! Depcrate_fmt_humantimetests {
() => {
// Module: crate::fmt::humantime
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Timestamp ; use crate :: TimestampPrecision ; # [test] fn test_display_timestamp () { let mut ts = Timestamp { time : std :: time :: SystemTime :: UNIX_EPOCH , precision : TimestampPrecision :: Nanos , } ; assert_eq ! ("1970-01-01T00:00:00.000000000Z" , format ! ("{ts}")) ; ts . precision = TimestampPrecision :: Micros ; assert_eq ! ("1970-01-01T00:00:00.000000Z" , format ! ("{ts}")) ; ts . precision = TimestampPrecision :: Millis ; assert_eq ! ("1970-01-01T00:00:00.000Z" , format ! ("{ts}")) ; ts . precision = TimestampPrecision :: Seconds ; assert_eq ! ("1970-01-01T00:00:00Z" , format ! ("{ts}")) ; } }
};
}

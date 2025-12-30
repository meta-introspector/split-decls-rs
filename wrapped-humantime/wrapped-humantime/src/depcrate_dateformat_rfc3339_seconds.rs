// Generated macro for format_rfc3339_seconds (function)
macro_rules! Depcrate_dateformat_rfc3339_seconds {
() => {
// Module: crate::date
// Provides: {"format_rfc3339_seconds"}
// Dependencies: {}
# [doc = " Format an RFC3339 timestamp `2018-02-14T00:28:07Z`"] # [doc = ""] # [doc = " This format always shows timestamp without fractional seconds."] # [doc = ""] # [doc = " The value is always UTC and ignores system timezone."] pub fn format_rfc3339_seconds (system_time : SystemTime) -> Rfc3339Timestamp { Rfc3339Timestamp (system_time , Precision :: Seconds) }
};
}

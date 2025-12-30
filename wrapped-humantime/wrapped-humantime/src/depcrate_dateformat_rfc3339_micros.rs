// Generated macro for format_rfc3339_micros (function)
macro_rules! Depcrate_dateformat_rfc3339_micros {
() => {
// Module: crate::date
// Provides: {"format_rfc3339_micros"}
// Dependencies: {}
# [doc = " Format an RFC3339 timestamp `2018-02-14T00:28:07.000000Z`"] # [doc = ""] # [doc = " This format always shows microseconds even if microsecond value is zero."] # [doc = ""] # [doc = " The value is always UTC and ignores system timezone."] pub fn format_rfc3339_micros (system_time : SystemTime) -> Rfc3339Timestamp { Rfc3339Timestamp (system_time , Precision :: Micros) }
};
}

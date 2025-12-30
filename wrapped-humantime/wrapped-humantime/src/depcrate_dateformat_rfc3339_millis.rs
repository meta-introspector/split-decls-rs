// Generated macro for format_rfc3339_millis (function)
macro_rules! Depcrate_dateformat_rfc3339_millis {
() => {
// Module: crate::date
// Provides: {"format_rfc3339_millis"}
// Dependencies: {}
# [doc = " Format an RFC3339 timestamp `2018-02-14T00:28:07.000Z`"] # [doc = ""] # [doc = " This format always shows milliseconds even if millisecond value is zero."] # [doc = ""] # [doc = " The value is always UTC and ignores system timezone."] pub fn format_rfc3339_millis (system_time : SystemTime) -> Rfc3339Timestamp { Rfc3339Timestamp (system_time , Precision :: Millis) }
};
}

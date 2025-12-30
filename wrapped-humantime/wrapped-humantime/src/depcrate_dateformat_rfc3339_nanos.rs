// Generated macro for format_rfc3339_nanos (function)
macro_rules! Depcrate_dateformat_rfc3339_nanos {
() => {
// Module: crate::date
// Provides: {"format_rfc3339_nanos"}
// Dependencies: {}
# [doc = " Format an RFC3339 timestamp `2018-02-14T00:28:07.000000000Z`"] # [doc = ""] # [doc = " This format always shows nanoseconds even if nanosecond value is zero."] # [doc = ""] # [doc = " The value is always UTC and ignores system timezone."] pub fn format_rfc3339_nanos (system_time : SystemTime) -> Rfc3339Timestamp { Rfc3339Timestamp (system_time , Precision :: Nanos) }
};
}

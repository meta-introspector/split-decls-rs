// Generated macro for format_rfc3339 (function)
macro_rules! Depcrate_dateformat_rfc3339 {
() => {
// Module: crate::date
// Provides: {"format_rfc3339"}
// Dependencies: {}
# [doc = " Format an RFC3339 timestamp `2018-02-14T00:28:07Z`"] # [doc = ""] # [doc = " This function formats timestamp with smart precision: i.e. if it has no"] # [doc = " fractional seconds, they aren't written at all. And up to nine digits if"] # [doc = " they are."] # [doc = ""] # [doc = " The value is always UTC and ignores system timezone."] pub fn format_rfc3339 (system_time : SystemTime) -> Rfc3339Timestamp { Rfc3339Timestamp (system_time , Precision :: Smart) }
};
}

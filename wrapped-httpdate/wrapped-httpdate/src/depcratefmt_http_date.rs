// Generated macro for fmt_http_date (function)
macro_rules! Depcratefmt_http_date {
() => {
// Module: crate
// Provides: {"fmt_http_date"}
// Dependencies: {}
# [doc = " Format a date to be used in a HTTP header field."] # [doc = ""] # [doc = " Dates are formatted as IMF-fixdate: `Fri, 15 May 2015 15:34:21 GMT`."] pub fn fmt_http_date (d : SystemTime) -> String { format ! ("{}" , HttpDate :: from (d)) }
};
}

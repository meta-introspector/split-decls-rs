// Generated macro for parse_http_date (function)
macro_rules! Depcrateparse_http_date {
() => {
// Module: crate
// Provides: {"parse_http_date"}
// Dependencies: {}
# [doc = " Parse a date from an HTTP header field."] # [doc = ""] # [doc = " Supports the preferred IMF-fixdate and the legacy RFC 805 and"] # [doc = " ascdate formats. Two digit years are mapped to dates between"] # [doc = " 1970 and 2069."] pub fn parse_http_date (s : & str) -> Result < SystemTime , Error > { s . parse :: < HttpDate > () . map (| d | d . into ()) }
};
}

// Generated macro for parse_rfc3339 (function)
macro_rules! Depcrate_dateparse_rfc3339 {
() => {
// Module: crate::date
// Provides: {"parse_rfc3339"}
// Dependencies: {}
# [doc = " Parse RFC3339 timestamp `2018-02-14T00:28:07Z`"] # [doc = ""] # [doc = " Supported features:"] # [doc = " - Any precision of fractional digits `2018-02-14T00:28:07.133Z`."] # [doc = " - The UTC timezone can be indicated with `Z` or `+00:00`."] # [doc = ""] # [doc = " Unsupported feature: localized timestamps. Only UTC is supported."] pub fn parse_rfc3339 (s : & str) -> Result < SystemTime , Error > { if s . len () < "2018-02-14T00:28:07Z" . len () { return Err (Error :: InvalidFormat) ; } let b = s . as_bytes () ; if b [10] != b'T' || (b . last () != Some (& b'Z') && ! s . ends_with ("+00:00")) { return Err (Error :: InvalidFormat) ; } parse_rfc3339_weak (s) }
};
}

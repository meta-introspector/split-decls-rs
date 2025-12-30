// Generated macro for date_time (module)
macro_rules! Depcrate_integrations_bsondate_time {
() => {
// Module: crate::integrations::bson
// Provides: {"date_time"}
// Dependencies: {}
mod date_time { use super :: DateTime ; pub (super) fn to_output (v : & DateTime) -> String { (* v) . try_to_rfc3339_string () . unwrap_or_else (| e | panic ! ("failed to format `DateTime` as RFC 3339: {e}")) } pub (super) fn from_input (s : & str) -> Result < DateTime , Box < str > > { DateTime :: parse_rfc3339_str (s) . map_err (| e | format ! ("Failed to parse `DateTime`: {e}") . into ()) } }
};
}

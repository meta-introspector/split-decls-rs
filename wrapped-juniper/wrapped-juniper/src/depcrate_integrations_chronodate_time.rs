// Generated macro for date_time (module)
macro_rules! Depcrate_integrations_chronodate_time {
() => {
// Module: crate::integrations::chrono
// Provides: {"date_time"}
// Dependencies: {}
mod date_time { use std :: fmt :: Display ; use chrono :: { FixedOffset , SecondsFormat , TimeZone , Utc } ; use super :: { DateTime , FromFixedOffset } ; pub (super) fn to_output < Tz > (v : & DateTime < Tz >) -> String where Tz : TimeZone , Tz :: Offset : Display , { v . with_timezone (& Utc) . to_rfc3339_opts (SecondsFormat :: AutoSi , true) } pub (super) fn from_input < Tz > (s : & str) -> Result < DateTime < Tz > , Box < str > > where Tz : TimeZone + FromFixedOffset , { DateTime :: < FixedOffset > :: parse_from_rfc3339 (s) . map (FromFixedOffset :: from_fixed_offset) . map_err (| e | format ! ("Invalid `DateTime`: {e}") . into ()) } }
};
}

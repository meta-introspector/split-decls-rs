// Generated macro for time_zone_or_utc_offset (module)
macro_rules! Depcrate_integrations_jifftime_zone_or_utc_offset {
() => {
// Module: crate::integrations::jiff
// Provides: {"time_zone_or_utc_offset"}
// Dependencies: {}
mod time_zone_or_utc_offset { use std :: fmt :: Display ; use super :: { TimeZoneOrUtcOffset , TimeZoneParsingError , utc_offset } ; use crate :: util :: Either ; # [doc = " Format of a [`TimeZoneOrUtcOffset`] scalar."] const FORMAT : & str = "%:Q" ; pub (super) fn to_output (v : & TimeZoneOrUtcOffset) -> impl Display { if let Some (name) = v . iana_name () { Either :: Left (name) } else { Either :: Right (jiff :: Zoned :: now () . with_time_zone (v . clone ()) . strftime (FORMAT) ,) } } pub (super) fn from_input (s : & str) -> Result < TimeZoneOrUtcOffset , Box < str > > { TimeZoneOrUtcOffset :: get (s) . map_err (TimeZoneParsingError :: InvalidTimeZone) . or_else (| _ | utc_offset :: utc_offset_from_str (s) . map (TimeZoneOrUtcOffset :: fixed)) . map_err (| e | format ! ("Invalid `TimeZoneOrUtcOffset`: {e}") . into ()) } }
};
}

// Generated macro for impl_1206 (impl)
macro_rules! Depcrate_integrations_jiffimpl_1206 {
() => {
// Module: crate::integrations::jiff
// Provides: {"impl_1206"}
// Dependencies: {}
impl str :: FromStr for TimeZone { type Err = TimeZoneParsingError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { let value = jiff :: tz :: TimeZone :: get (value) . map_err (TimeZoneParsingError :: InvalidTimeZone) ? ; value . try_into () } }
};
}

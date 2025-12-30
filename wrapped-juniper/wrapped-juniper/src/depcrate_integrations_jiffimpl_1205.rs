// Generated macro for impl_1205 (impl)
macro_rules! Depcrate_integrations_jiffimpl_1205 {
() => {
// Module: crate::integrations::jiff
// Provides: {"impl_1205"}
// Dependencies: {}
impl TryFrom < jiff :: tz :: TimeZone > for TimeZone { type Error = TimeZoneParsingError ; fn try_from (value : jiff :: tz :: TimeZone) -> Result < Self , Self :: Error > { if value . iana_name () . is_none () { return Err (TimeZoneParsingError :: MissingIanaName (value)) ; } Ok (Self (value)) } }
};
}

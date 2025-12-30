// Generated macro for FormatTimeZone (trait)
macro_rules! Depcrate_format_time_zoneFormatTimeZone {
() => {
// Module: crate::format::time_zone
// Provides: {"FormatTimeZone"}
// Dependencies: {}
pub (super) trait FormatTimeZone { # [doc = " Tries to write the timezone to the sink. If a DateTimeError is returned, the sink"] # [doc = " has not been touched, so another format can be attempted."] fn format < W : writeable :: PartsWrite + ? Sized > (& self , sink : & mut W , input : & DateTimeInputUnchecked , data_payloads : TimeZoneDataPayloadsBorrowed , fdf : Option < & DecimalFormatter > ,) -> Result < Result < () , FormatTimeZoneError > , fmt :: Error > ; }
};
}

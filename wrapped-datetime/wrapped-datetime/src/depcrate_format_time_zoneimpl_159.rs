// Generated macro for impl_159 (impl)
macro_rules! Depcrate_format_time_zoneimpl_159 {
() => {
// Module: crate::format::time_zone
// Provides: {"impl_159"}
// Dependencies: {}
impl FormatTimeZone for Bcp47IdFormat { fn format < W : writeable :: PartsWrite + ? Sized > (& self , sink : & mut W , input : & DateTimeInputUnchecked , _data_payloads : TimeZoneDataPayloadsBorrowed , _fdf : Option < & DecimalFormatter > ,) -> Result < Result < () , FormatTimeZoneError > , fmt :: Error > { let time_zone_id = input . zone_id . unwrap_or (TimeZone :: UNKNOWN) ; sink . write_str (time_zone_id . as_str ()) ? ; Ok (Ok (())) } }
};
}

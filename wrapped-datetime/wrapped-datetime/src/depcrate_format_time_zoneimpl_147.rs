// Generated macro for impl_147 (impl)
macro_rules! Depcrate_format_time_zoneimpl_147 {
() => {
// Module: crate::format::time_zone
// Provides: {"impl_147"}
// Dependencies: {}
impl FormatTimeZone for GenericLocationFormat { # [doc = " Writes the time zone in generic location format as defined by the UTS-35 spec."] # [doc = " e.g. France Time"] # [doc = " <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>"] fn format < W : writeable :: PartsWrite + ? Sized > (& self , sink : & mut W , input : & DateTimeInputUnchecked , data_payloads : TimeZoneDataPayloadsBorrowed , _decimal_formatter : Option < & DecimalFormatter > ,) -> Result < Result < () , FormatTimeZoneError > , fmt :: Error > { let Some (time_zone_id) = input . zone_id else { return Ok (Err (FormatTimeZoneError :: MissingInputField (MissingInputFieldKind :: TimeZoneId ,))) ; } ; let Some (locations) = data_payloads . locations else { return Ok (Err (FormatTimeZoneError :: NamesNotLoaded)) ; } ; let Some (locations_root) = data_payloads . locations_root else { return Ok (Err (FormatTimeZoneError :: NamesNotLoaded)) ; } ; if let (Some (mz_periods) , Some (offset) , Some (timestamp)) = (data_payloads . mz_periods , input . zone_offset , input . zone_name_timestamp ,) { if let Some ((os , _)) = mz_periods . get (time_zone_id , timestamp) { if offset != os . standard && Some (offset) != os . daylight { return Ok (Err (FormatTimeZoneError :: Fallback)) ; } ; } } let Some (location) = locations . locations . get (& time_zone_id) . or_else (| | locations_root . locations . get (& time_zone_id)) else { return Ok (Err (FormatTimeZoneError :: Fallback)) ; } ; locations . pattern_generic . interpolate ([location]) . write_to (sink) ? ; Ok (Ok (())) } }
};
}

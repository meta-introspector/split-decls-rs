// Generated macro for impl_156 (impl)
macro_rules! Depcrate_format_time_zoneimpl_156 {
() => {
// Module: crate::format::time_zone
// Provides: {"impl_156"}
// Dependencies: {}
impl FormatTimeZone for Iso8601Format { # [doc = " Writes a [`UtcOffset`](crate::input::UtcOffset) in ISO-8601 format according to the"] # [doc = " given formatting options."] # [doc = ""] # [doc = " [`IsoFormat`] determines whether the format should be Basic or Extended,"] # [doc = " and whether a zero-offset should be formatted numerically or with"] # [doc = " The UTC indicator: \"Z\""] # [doc = " - Basic    e.g. +0800"] # [doc = " - Extended e.g. +08:00"] # [doc = ""] # [doc = " [`Minutes`] can be required or optional."] # [doc = " [`Seconds`] can be optional or never."] fn format < W : writeable :: PartsWrite + ? Sized > (& self , sink : & mut W , input : & DateTimeInputUnchecked , _data_payloads : TimeZoneDataPayloadsBorrowed , _fdf : Option < & DecimalFormatter > ,) -> Result < Result < () , FormatTimeZoneError > , fmt :: Error > { let Some (offset) = input . zone_offset else { sink . write_str ("+?") ? ; return Ok (Ok (())) ; } ; self . format_infallible (sink , offset) . map (| () | Ok (())) } }
};
}

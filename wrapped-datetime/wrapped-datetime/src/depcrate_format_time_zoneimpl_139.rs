// Generated macro for impl_139 (impl)
macro_rules! Depcrate_format_time_zoneimpl_139 {
() => {
// Module: crate::format::time_zone
// Provides: {"impl_139"}
// Dependencies: {}
impl FormatTimeZone for TimeZoneFormatterUnit { fn format < W : writeable :: PartsWrite + ? Sized > (& self , sink : & mut W , input : & DateTimeInputUnchecked , data_payloads : TimeZoneDataPayloadsBorrowed , fdf : Option < & DecimalFormatter > ,) -> Result < Result < () , FormatTimeZoneError > , fmt :: Error > { match * self { Self :: GenericNonLocation (length) => { GenericNonLocationFormat (length) . format (sink , input , data_payloads , fdf) } Self :: SpecificNonLocation (length) => { SpecificNonLocationFormat (length) . format (sink , input , data_payloads , fdf) } Self :: GenericLocation => GenericLocationFormat . format (sink , input , data_payloads , fdf) , Self :: ExemplarCity => ExemplarCityFormat . format (sink , input , data_payloads , fdf) , Self :: GenericPartialLocation (length) => { GenericPartialLocationFormat (length) . format (sink , input , data_payloads , fdf) } Self :: LocalizedOffset (length) => { LocalizedOffsetFormat (length) . format (sink , input , data_payloads , fdf) } Self :: Iso8601 (iso) => iso . format (sink , input , data_payloads , fdf) , Self :: Bcp47Id => Bcp47IdFormat . format (sink , input , data_payloads , fdf) , } } }
};
}

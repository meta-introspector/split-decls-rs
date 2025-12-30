// Generated macro for impl_157 (impl)
macro_rules! Depcrate_format_time_zoneimpl_157 {
() => {
// Module: crate::format::time_zone
// Provides: {"impl_157"}
// Dependencies: {}
impl Iso8601Format { pub (crate) fn format_infallible < W : writeable :: PartsWrite + ? Sized > (self , sink : & mut W , offset : UtcOffset ,) -> Result < () , fmt :: Error > { if offset . is_zero () && self . z { return sink . write_char ('Z') ; } { let mut fd = Decimal :: from (offset . hours_part ()) . with_sign_display (fixed_decimal :: SignDisplay :: Always) ; fd . pad_start (2) ; fd } . write_to (sink) ? ; if self . minutes == Minutes :: Required || (self . minutes == Minutes :: Optional && offset . minutes_part () != 0) { if self . extended { sink . write_char (':') ? ; } { let mut fd = Decimal :: from (offset . minutes_part ()) ; fd . pad_start (2) ; fd } . write_to (sink) ? ; } if self . seconds == Seconds :: Optional && offset . seconds_part () != 0 { if self . extended { sink . write_char (':') ? ; } { let mut fd = Decimal :: from (offset . seconds_part ()) ; fd . pad_start (2) ; fd } . write_to (sink) ? ; } Ok (()) } }
};
}

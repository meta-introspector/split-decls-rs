// Generated macro for write_rfc2822 (function)
macro_rules! Depcrate_format_formattingwrite_rfc2822 {
() => {
// Module: crate::format::formatting
// Provides: {"write_rfc2822"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [doc = " write datetimes like `Tue, 1 Jul 2003 10:52:37 +0200`, same as `%a, %d %b %Y %H:%M:%S %z`"] pub (crate) fn write_rfc2822 (w : & mut (impl Write + ? Sized) , dt : NaiveDateTime , off : FixedOffset ,) -> fmt :: Result { let year = dt . year () ; if ! (0 ..= 9999) . contains (& year) { return Err (fmt :: Error) ; } let english = default_locale () ; w . write_str (short_weekdays (english) [dt . weekday () . num_days_from_sunday () as usize]) ? ; w . write_str (", ") ? ; let day = dt . day () ; if day < 10 { w . write_char ((b'0' + day as u8) as char) ? ; } else { write_hundreds (w , day as u8) ? ; } w . write_char (' ') ? ; w . write_str (short_months (english) [dt . month0 () as usize]) ? ; w . write_char (' ') ? ; write_hundreds (w , (year / 100) as u8) ? ; write_hundreds (w , (year % 100) as u8) ? ; w . write_char (' ') ? ; let (hour , min , sec) = dt . time () . hms () ; write_hundreds (w , hour as u8) ? ; w . write_char (':') ? ; write_hundreds (w , min as u8) ? ; w . write_char (':') ? ; let sec = sec + dt . nanosecond () / 1_000_000_000 ; write_hundreds (w , sec as u8) ? ; w . write_char (' ') ? ; OffsetFormat { precision : OffsetPrecision :: Minutes , colons : Colons :: None , allow_zulu : false , padding : Pad :: Zero , } . format (w , off) }
};
}

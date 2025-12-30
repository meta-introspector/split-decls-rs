// Generated macro for try_write_number (function)
macro_rules! Depcrate_format_datetimetry_write_number {
() => {
// Module: crate::format::datetime
// Provides: {"try_write_number"}
// Dependencies: {}
# [doc = " Apply length to input number and write to result using decimal_formatter."] fn try_write_number < W > (part : Part , w : & mut W , decimal_formatter : Option < & DecimalFormatter > , mut num : Decimal , length : FieldLength ,) -> Result < Result < () , FormattedDateTimePatternError > , fmt :: Error > where W : writeable :: PartsWrite + ? Sized , { num . pad_start (length . to_len () as i16) ; if let Some (fdf) = decimal_formatter { w . with_part (part , | w | fdf . format (& num) . write_to_parts (w)) ? ; Ok (Ok (())) } else { w . with_part (part , | w | { w . with_part (writeable :: Part :: ERROR , | r | num . write_to_parts (r)) }) ? ; Ok (Err (FormattedDateTimePatternError :: DecimalFormatterNotLoaded ,)) } }
};
}

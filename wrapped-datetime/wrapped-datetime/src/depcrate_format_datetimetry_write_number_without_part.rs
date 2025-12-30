// Generated macro for try_write_number_without_part (function)
macro_rules! Depcrate_format_datetimetry_write_number_without_part {
() => {
// Module: crate::format::datetime
// Provides: {"try_write_number_without_part"}
// Dependencies: {}
# [doc = " Apply length to input number and write to result using decimal_formatter."] # [doc = " Don't annotate it with a part."] fn try_write_number_without_part < W > (w : & mut W , decimal_formatter : Option < & DecimalFormatter > , mut num : Decimal , length : FieldLength ,) -> Result < Result < () , FormattedDateTimePatternError > , fmt :: Error > where W : writeable :: PartsWrite + ? Sized , { num . pad_start (length . to_len () as i16) ; if let Some (fdf) = decimal_formatter { fdf . format (& num) . write_to (w) ? ; Ok (Ok (())) } else { w . with_part (writeable :: Part :: ERROR , | r | num . write_to (r)) ? ; Ok (Err (FormattedDateTimePatternError :: DecimalFormatterNotLoaded ,)) } }
};
}

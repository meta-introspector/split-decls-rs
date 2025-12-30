// Generated macro for try_write_pattern_items (function)
macro_rules! Depcrate_format_datetimetry_write_pattern_items {
() => {
// Module: crate::format::datetime
// Provides: {"try_write_pattern_items"}
// Dependencies: {}
pub (crate) fn try_write_pattern_items < W > (pattern_metadata : PatternMetadata , pattern_items : impl Iterator < Item = PatternItem > , input : & DateTimeInputUnchecked , datetime_names : & RawDateTimeNamesBorrowed , decimal_formatter : Option < & DecimalFormatter > , w : & mut W ,) -> Result < Result < () , FormattedDateTimePatternError > , fmt :: Error > where W : writeable :: PartsWrite + ? Sized , { let mut r = Ok (()) ; for item in pattern_items { match item { PatternItem :: Literal (ch) => w . write_char (ch) ? , PatternItem :: Field (field) => { r = r . and (try_write_field (field , pattern_metadata , input , datetime_names , decimal_formatter , w ,) ?) ; } } } Ok (r) }
};
}

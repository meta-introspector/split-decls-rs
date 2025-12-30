// Generated macro for short_or_long_month0 (function)
macro_rules! Depcrate_format_scanshort_or_long_month0 {
() => {
// Module: crate::format::scan
// Provides: {"short_or_long_month0"}
// Dependencies: {}
# [doc = " Tries to parse the month index (0 through 11) with short or long month names."] # [doc = " It prefers long month names to short month names when both are possible."] pub (super) fn short_or_long_month0 (s : & str) -> ParseResult < (& str , u8) > { static LONG_MONTH_SUFFIXES : [& [u8] ; 12] = [b"uary" , b"ruary" , b"ch" , b"il" , b"" , b"e" , b"y" , b"ust" , b"tember" , b"ober" , b"ember" , b"ember" ,] ; let (mut s , month0) = short_month0 (s) ? ; let suffix = LONG_MONTH_SUFFIXES [month0 as usize] ; if s . len () >= suffix . len () && s . as_bytes () [.. suffix . len ()] . eq_ignore_ascii_case (suffix) { s = & s [suffix . len () ..] ; } Ok ((s , month0)) }
};
}

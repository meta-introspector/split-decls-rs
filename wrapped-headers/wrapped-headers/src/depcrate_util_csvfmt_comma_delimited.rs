// Generated macro for fmt_comma_delimited (function)
macro_rules! Depcrate_util_csvfmt_comma_delimited {
() => {
// Module: crate::util::csv
// Provides: {"fmt_comma_delimited"}
// Dependencies: {}
# [doc = " Format an array into a comma-delimited string."] pub (crate) fn fmt_comma_delimited < T : fmt :: Display > (f : & mut fmt :: Formatter , mut iter : impl Iterator < Item = T > ,) -> fmt :: Result { if let Some (part) = iter . next () { fmt :: Display :: fmt (& part , f) ? ; } for part in iter { f . write_str (", ") ? ; fmt :: Display :: fmt (& part , f) ? ; } Ok (()) }
};
}

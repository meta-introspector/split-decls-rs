// Generated macro for fmt_with_thousand_sep (function)
macro_rules! Depcrate_printerfmt_with_thousand_sep {
() => {
// Module: crate::printer
// Provides: {"fmt_with_thousand_sep"}
// Dependencies: {}
# [doc = " Formats the given integer with `,` as thousand separator."] pub fn fmt_with_thousand_sep (mut v : u64) -> String { let mut out = String :: new () ; while v >= 1000 { out = format ! (",{:03}{}" , v % 1000 , out) ; v /= 1000 ; } out = format ! ("{}{}" , v , out) ; out }
};
}

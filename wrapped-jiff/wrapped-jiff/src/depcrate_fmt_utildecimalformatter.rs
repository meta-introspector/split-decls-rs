// Generated macro for DecimalFormatter (struct)
macro_rules! Depcrate_fmt_utilDecimalFormatter {
() => {
// Module: crate::fmt::util
// Provides: {"DecimalFormatter"}
// Dependencies: {}
# [doc = " A simple formatter for converting `i64` values to ASCII byte strings."] # [doc = ""] # [doc = " This avoids going through the formatting machinery which seems to"] # [doc = " substantially slow things down."] # [doc = ""] # [doc = " The `itoa` crate does the same thing as this formatter, but is a bit"] # [doc = " faster. We roll our own which is a bit slower, but gets us enough of a win"] # [doc = " to be satisfied with and with (almost) pure safe code."] # [doc = ""] # [doc = " By default, this only includes the sign if it's negative. To always include"] # [doc = " the sign, set `force_sign` to `true`."] # [derive (Clone , Copy , Debug)] pub (crate) struct DecimalFormatter { force_sign : Option < bool > , minimum_digits : u8 , padding_byte : u8 , }
};
}

// Generated macro for is_decimal_separator (function)
macro_rules! Depcrate_parsers_grammaris_decimal_separator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_decimal_separator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `DecimalSeparator`."] # [inline] pub (crate) const fn is_decimal_separator (ch : u8) -> bool { ch == b'.' || ch == b',' }
};
}

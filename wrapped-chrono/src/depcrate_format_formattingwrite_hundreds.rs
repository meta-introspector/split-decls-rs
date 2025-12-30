// Generated macro for write_hundreds (function)
macro_rules! Depcrate_format_formattingwrite_hundreds {
() => {
// Module: crate::format::formatting
// Provides: {"write_hundreds"}
// Dependencies: {}
# [doc = " Equivalent to `{:02}` formatting for n < 100."] pub (crate) fn write_hundreds (w : & mut (impl Write + ? Sized) , n : u8) -> fmt :: Result { if n >= 100 { return Err (fmt :: Error) ; } let tens = b'0' + n / 10 ; let ones = b'0' + n % 10 ; w . write_char (tens as char) ? ; w . write_char (ones as char) }
};
}

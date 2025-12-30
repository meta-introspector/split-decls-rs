// Generated macro for EscapeOptions (struct)
macro_rules! Depcrate_escapeEscapeOptions {
() => {
// Module: crate::escape
// Provides: {"EscapeOptions"}
// Dependencies: {}
# [derive (Copy , Clone)] pub (crate) struct EscapeOptions { # [doc = " Produce \\'."] pub escape_single_quote : bool , # [doc = " Produce \\\"."] pub escape_double_quote : bool , # [doc = " Produce \\x escapes for non-ASCII, and use \\x rather than \\u for ASCII"] # [doc = " control characters."] pub escape_nonascii : bool , }
};
}

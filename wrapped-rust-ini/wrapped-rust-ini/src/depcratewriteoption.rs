// Generated macro for WriteOption (struct)
macro_rules! DepcrateWriteOption {
() => {
// Module: crate
// Provides: {"WriteOption"}
// Dependencies: {}
# [doc = " Writing configuration"] # [derive (Debug , Clone)] pub struct WriteOption { # [doc = " Policies about how to escape characters"] pub escape_policy : EscapePolicy , # [doc = " Newline style"] pub line_separator : LineSeparator , # [doc = " Key value separator"] pub kv_separator : & 'static str , }
};
}

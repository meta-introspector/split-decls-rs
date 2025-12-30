// Generated macro for parse (function)
macro_rules! Depcrateparse {
() => {
// Module: crate
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Create a [`Pattern`] by parsing `text` or return `None` if `text` is empty."] # [doc = ""] # [doc = " Note that"] pub fn parse (text : impl AsRef < [u8] >) -> Option < Pattern > { Pattern :: from_bytes (text . as_ref ()) }
};
}

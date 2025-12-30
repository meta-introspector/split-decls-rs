// Generated macro for parse (function)
macro_rules! Depcrateparse {
() => {
// Module: crate
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Parse the given `buf` of bytes line by line into mapping [Entries][Entry]."] # [doc = ""] # [doc = " Errors may occur per line, but it's up to the caller to stop iteration when"] # [doc = " one is encountered."] pub fn parse (buf : & [u8]) -> parse :: Lines < '_ > { parse :: Lines :: new (buf) }
};
}

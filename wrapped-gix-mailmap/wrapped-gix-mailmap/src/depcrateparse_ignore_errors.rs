// Generated macro for parse_ignore_errors (function)
macro_rules! Depcrateparse_ignore_errors {
() => {
// Module: crate
// Provides: {"parse_ignore_errors"}
// Dependencies: {}
# [doc = " Similar to [parse()], but will skip all lines that didn't parse correctly, silently squelching all errors."] pub fn parse_ignore_errors (buf : & [u8]) -> impl Iterator < Item = Entry < '_ > > { parse (buf) . filter_map (Result :: ok) }
};
}

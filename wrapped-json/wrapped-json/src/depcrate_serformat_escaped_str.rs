// Generated macro for format_escaped_str (function)
macro_rules! Depcrate_serformat_escaped_str {
() => {
// Module: crate::ser
// Provides: {"format_escaped_str"}
// Dependencies: {}
fn format_escaped_str < W , F > (writer : & mut W , formatter : & mut F , value : & str) -> io :: Result < () > where W : ? Sized + io :: Write , F : ? Sized + Formatter , { tri ! (formatter . begin_string (writer)) ; tri ! (format_escaped_str_contents (writer , formatter , value)) ; formatter . end_string (writer) }
};
}

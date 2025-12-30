// Generated macro for format_error_message (function)
macro_rules! Depcrate_error_formatformat_error_message {
() => {
// Module: crate::error::format
// Provides: {"format_error_message"}
// Dependencies: {}
pub (crate) fn format_error_message (message : & str , styles : & Styles , cmd : Option < & Command > , usage : Option < & StyledStr > ,) -> StyledStr { let mut styled = StyledStr :: new () ; start_error (& mut styled , styles) ; styled . push_str (message) ; if let Some (usage) = usage { put_usage (& mut styled , usage) ; } if let Some (cmd) = cmd { try_help (& mut styled , styles , get_help_flag (cmd) . as_deref ()) ; } styled }
};
}

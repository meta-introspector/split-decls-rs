// Generated macro for get_user_help_flag (function)
macro_rules! Depcrate_error_formatget_user_help_flag {
() => {
// Module: crate::error::format
// Provides: {"get_user_help_flag"}
// Dependencies: {}
fn get_user_help_flag (cmd : & Command) -> Option < String > { let arg = cmd . get_arguments () . find (| arg | match arg . get_action () { ArgAction :: Help | ArgAction :: HelpShort | ArgAction :: HelpLong => true , ArgAction :: Append | ArgAction :: Count | ArgAction :: SetTrue | ArgAction :: SetFalse | ArgAction :: Set | ArgAction :: Version => false , }) ? ; arg . get_long () . map (| long | format ! ("--{long}")) . or_else (| | arg . get_short () . map (| short | format ! ("-{short}"))) }
};
}

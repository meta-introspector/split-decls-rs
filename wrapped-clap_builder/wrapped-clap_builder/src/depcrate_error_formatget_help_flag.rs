// Generated macro for get_help_flag (function)
macro_rules! Depcrate_error_formatget_help_flag {
() => {
// Module: crate::error::format
// Provides: {"get_help_flag"}
// Dependencies: {}
pub (crate) fn get_help_flag (cmd : & Command) -> Option < Cow < 'static , str > > { if ! cmd . is_disable_help_flag_set () { Some (Cow :: Borrowed ("--help")) } else if let Some (flag) = get_user_help_flag (cmd) { Some (Cow :: Owned (flag)) } else if cmd . has_subcommands () && ! cmd . is_disable_help_subcommand_set () { Some (Cow :: Borrowed ("help")) } else { None } }
};
}

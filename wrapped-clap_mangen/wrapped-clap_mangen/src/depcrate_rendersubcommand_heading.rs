// Generated macro for subcommand_heading (function)
macro_rules! Depcrate_rendersubcommand_heading {
() => {
// Module: crate::render
// Provides: {"subcommand_heading"}
// Dependencies: {}
pub (crate) fn subcommand_heading (cmd : & clap :: Command) -> & str { match cmd . get_subcommand_help_heading () { Some (title) => title , None => "SUBCOMMANDS" , } }
};
}

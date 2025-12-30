// Generated macro for populate_command_candidate (function)
macro_rules! Depcrate_engine_completepopulate_command_candidate {
() => {
// Module: crate::engine::complete
// Provides: {"populate_command_candidate"}
// Dependencies: {}
fn populate_command_candidate (candidate : CompletionCandidate , cmd : & clap :: Command , subcommand : & clap :: Command ,) -> CompletionCandidate { candidate . help (subcommand . get_about () . cloned ()) . id (Some (format ! ("command::{}" , subcommand . get_name ()))) . tag (Some (cmd . get_subcommand_help_heading () . unwrap_or ("Commands") . to_owned () . into () ,)) . display_order (Some (subcommand . get_display_order ())) . hide (subcommand . is_hide_set ()) }
};
}

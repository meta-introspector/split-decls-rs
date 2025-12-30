// Generated macro for populate_arg_candidate (function)
macro_rules! Depcrate_engine_completepopulate_arg_candidate {
() => {
// Module: crate::engine::complete
// Provides: {"populate_arg_candidate"}
// Dependencies: {}
fn populate_arg_candidate (candidate : CompletionCandidate , arg : & clap :: Arg) -> CompletionCandidate { candidate . help (arg . get_help () . cloned ()) . id (Some (format ! ("arg::{}" , arg . get_id ()))) . tag (Some (arg . get_help_heading () . unwrap_or ("Options") . to_owned () . into () ,)) . display_order (Some (arg . get_display_order ())) . hide (arg . is_hide_set ()) }
};
}

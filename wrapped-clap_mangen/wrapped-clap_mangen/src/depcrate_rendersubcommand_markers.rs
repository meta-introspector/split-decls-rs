// Generated macro for subcommand_markers (function)
macro_rules! Depcrate_rendersubcommand_markers {
() => {
// Module: crate::render
// Provides: {"subcommand_markers"}
// Dependencies: {}
fn subcommand_markers (cmd : & clap :: Command) -> (& 'static str , & 'static str) { markers (cmd . is_subcommand_required_set ()) }
};
}

// Generated macro for subcommand_sort_key (function)
macro_rules! Depcrate_rendersubcommand_sort_key {
() => {
// Module: crate::render
// Provides: {"subcommand_sort_key"}
// Dependencies: {}
fn subcommand_sort_key (command : & clap :: Command) -> (usize , & str) { (command . get_display_order () , command . get_name ()) }
};
}

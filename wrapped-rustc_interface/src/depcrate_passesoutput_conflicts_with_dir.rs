// Generated macro for output_conflicts_with_dir (function)
macro_rules! Depcrate_passesoutput_conflicts_with_dir {
() => {
// Module: crate::passes
// Provides: {"output_conflicts_with_dir"}
// Dependencies: {}
fn output_conflicts_with_dir (output_paths : & [PathBuf]) -> Option < & PathBuf > { output_paths . iter () . find (| output_path | output_path . is_dir ()) }
};
}

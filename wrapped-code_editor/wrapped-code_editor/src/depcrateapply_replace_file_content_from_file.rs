// Generated macro for apply_replace_file_content_from_file (function)
macro_rules! Depcrateapply_replace_file_content_from_file {
() => {
// Module: crate
// Provides: {"apply_replace_file_content_from_file"}
// Dependencies: {}
fn apply_replace_file_content_from_file (target_file : & PathBuf , source_file : & PathBuf) -> Result < () > { let new_content = fs :: read_to_string (source_file) . with_context (| | format ! ("Failed to read source file: {:?}" , source_file)) ? ; fs :: write (target_file , new_content) . with_context (| | format ! ("Failed to write modified file: {:?}" , target_file)) ? ; Ok (()) }
};
}

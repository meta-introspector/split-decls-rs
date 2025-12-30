// Generated macro for first_file_from_config_with_origin (function)
macro_rules! Depcrate_env_gitfirst_file_from_config_with_origin {
() => {
// Module: crate::env::git
// Provides: {"first_file_from_config_with_origin"}
// Dependencies: {}
fn first_file_from_config_with_origin (source : & BStr) -> Option < & BStr > { let file = source . strip_prefix (b"file:") ? ; let end_pos = file . find_byte (b'\0') ? ; file [.. end_pos] . as_bstr () . into () }
};
}

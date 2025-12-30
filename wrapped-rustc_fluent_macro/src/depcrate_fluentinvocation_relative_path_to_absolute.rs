// Generated macro for invocation_relative_path_to_absolute (function)
macro_rules! Depcrate_fluentinvocation_relative_path_to_absolute {
() => {
// Module: crate::fluent
// Provides: {"invocation_relative_path_to_absolute"}
// Dependencies: {}
# [doc = " Helper function for returning an absolute path for macro-invocation relative file paths."] # [doc = ""] # [doc = " If the input is already absolute, then the input is returned. If the input is not absolute,"] # [doc = " then it is appended to the directory containing the source file with this macro invocation."] fn invocation_relative_path_to_absolute (span : Span , path : & str) -> PathBuf { let path = Path :: new (path) ; if path . is_absolute () { path . to_path_buf () } else { let mut source_file_path = span . local_file () . unwrap () ; source_file_path . pop () ; source_file_path . push (path) ; source_file_path } }
};
}

// Generated macro for android_clang_compiler_uses_target_arg_internally (function)
macro_rules! Depcrateandroid_clang_compiler_uses_target_arg_internally {
() => {
// Module: crate
// Provides: {"android_clang_compiler_uses_target_arg_internally"}
// Dependencies: {}
fn android_clang_compiler_uses_target_arg_internally (clang_path : & Path) -> bool { if let Some (filename) = clang_path . file_name () { if let Some (filename_str) = filename . to_str () { if let Some (idx) = filename_str . rfind ('-') { return filename_str . split_at (idx) . 0 . contains ("android") ; } } } false }
};
}

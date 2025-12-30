// Generated macro for stamp_file_path (function)
macro_rules! Depcratestamp_file_path {
() => {
// Module: crate
// Provides: {"stamp_file_path"}
// Dependencies: {}
# [doc = " The path of the `stamp` file that gets created or updated whenever a"] # [doc = " particular test completes successfully."] fn stamp_file_path (config : & Config , testpaths : & TestPaths , revision : Option < & str >) -> Utf8PathBuf { output_base_dir (config , testpaths , revision) . join ("stamp") }
};
}

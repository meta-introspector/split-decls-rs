// Generated macro for output_relative_path (function)
macro_rules! Depcrate_commonoutput_relative_path {
() => {
// Module: crate::common
// Provides: {"output_relative_path"}
// Dependencies: {}
# [doc = " Absolute path to the directory where all output for all tests in the given `relative_dir` group"] # [doc = " should reside. Example:"] # [doc = ""] # [doc = " ```text"] # [doc = " /path/to/build/host-tuple/test/ui/relative/"] # [doc = " ```"] # [doc = ""] # [doc = " This is created early when tests are collected to avoid race conditions."] pub fn output_relative_path (config : & Config , relative_dir : & Utf8Path) -> Utf8PathBuf { config . build_test_suite_root . join (relative_dir) }
};
}

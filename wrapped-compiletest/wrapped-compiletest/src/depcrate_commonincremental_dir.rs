// Generated macro for incremental_dir (function)
macro_rules! Depcrate_commonincremental_dir {
() => {
// Module: crate::common
// Provides: {"incremental_dir"}
// Dependencies: {}
# [doc = " Absolute path to the directory to use for incremental compilation. Example:"] # [doc = "   /path/to/build/host-tuple/test/ui/relative/testname.mode/testname.inc"] pub fn incremental_dir (config : & Config , testpaths : & TestPaths , revision : Option < & str > ,) -> Utf8PathBuf { output_base_name (config , testpaths , revision) . with_extension ("inc") }
};
}

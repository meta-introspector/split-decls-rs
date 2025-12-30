// Generated macro for output_base_dir (function)
macro_rules! Depcrate_commonoutput_base_dir {
() => {
// Module: crate::common
// Provides: {"output_base_dir"}
// Dependencies: {}
# [doc = " Absolute path to the directory where all output for the given"] # [doc = " test/revision should reside. Example:"] # [doc = "   /path/to/build/host-tuple/test/ui/relative/testname.revision.mode/"] pub fn output_base_dir (config : & Config , testpaths : & TestPaths , revision : Option < & str > ,) -> Utf8PathBuf { output_relative_path (config , & testpaths . relative_dir) . join (output_testname_unique (config , testpaths , revision)) }
};
}

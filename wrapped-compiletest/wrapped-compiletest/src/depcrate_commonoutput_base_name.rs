// Generated macro for output_base_name (function)
macro_rules! Depcrate_commonoutput_base_name {
() => {
// Module: crate::common
// Provides: {"output_base_name"}
// Dependencies: {}
# [doc = " Absolute path to the base filename used as output for the given"] # [doc = " test/revision. Example:"] # [doc = "   /path/to/build/host-tuple/test/ui/relative/testname.revision.mode/testname"] pub fn output_base_name (config : & Config , testpaths : & TestPaths , revision : Option < & str > ,) -> Utf8PathBuf { output_base_dir (config , testpaths , revision) . join (testpaths . file . file_stem () . unwrap ()) }
};
}

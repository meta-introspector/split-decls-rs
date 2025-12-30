// Generated macro for output_testname_unique (function)
macro_rules! Depcrate_commonoutput_testname_unique {
() => {
// Module: crate::common
// Provides: {"output_testname_unique"}
// Dependencies: {}
# [doc = " Generates a unique name for the test, such as `testname.revision.mode`."] pub fn output_testname_unique (config : & Config , testpaths : & TestPaths , revision : Option < & str > ,) -> Utf8PathBuf { let mode = config . compare_mode . as_ref () . map_or ("" , | m | m . to_str ()) ; let debugger = config . debugger . as_ref () . map_or ("" , | m | m . to_str ()) ; Utf8PathBuf :: from (& testpaths . file . file_stem () . unwrap ()) . with_extra_extension (config . mode . output_dir_disambiguator ()) . with_extra_extension (revision . unwrap_or ("")) . with_extra_extension (mode) . with_extra_extension (debugger) }
};
}

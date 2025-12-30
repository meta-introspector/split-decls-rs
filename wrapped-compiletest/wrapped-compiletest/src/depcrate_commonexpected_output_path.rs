// Generated macro for expected_output_path (function)
macro_rules! Depcrate_commonexpected_output_path {
() => {
// Module: crate::common
// Provides: {"expected_output_path"}
// Dependencies: {}
# [doc = " Used by `ui` tests to generate things like `foo.stderr` from `foo.rs`."] pub fn expected_output_path (testpaths : & TestPaths , revision : Option < & str > , compare_mode : & Option < CompareMode > , kind : & str ,) -> Utf8PathBuf { assert ! (UI_EXTENSIONS . contains (& kind)) ; let mut parts = Vec :: new () ; if let Some (x) = revision { parts . push (x) ; } if let Some (ref x) = * compare_mode { parts . push (x . to_str ()) ; } parts . push (kind) ; let extension = parts . join (".") ; testpaths . file . with_extension (extension) }
};
}

// Generated macro for path (function)
macro_rules! Depcrate_path_helperspath {
() => {
// Module: crate::path_helpers
// Provides: {"path"}
// Dependencies: {}
# [doc = " Construct a `PathBuf` relative to the current working directory by joining `cwd()` with the"] # [doc = " relative path. This is mostly a convenience helper so the test writer does not need to write"] # [doc = " `PathBuf::from(path_like_string)`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use run_make_support::path;"] # [doc = " let p = path(\"support_file.txt\");"] # [doc = " ```"] pub fn path < P : AsRef < Path > > (p : P) -> PathBuf { cwd () . join (p . as_ref ()) }
};
}

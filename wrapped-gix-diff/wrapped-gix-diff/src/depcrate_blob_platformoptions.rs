// Generated macro for Options (struct)
macro_rules! Depcrate_blob_platformOptions {
() => {
// Module: crate::blob::platform
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for use in [Platform::new()]."] # [derive (Default , Copy , Clone)] pub struct Options { # [doc = " The algorithm to use when diffing."] # [doc = " If unset, it uses the [default algorithm](Algorithm::default())."] pub algorithm : Option < Algorithm > , # [doc = " If `true`, default `false`, then an external `diff` configured using gitattributes and drivers,"] # [doc = " will cause the built-in diff [to be skipped](prepare_diff::Operation::ExternalCommand)."] # [doc = " Otherwise, the internal diff is called despite the configured external diff, which is"] # [doc = " typically what callers expect by default."] pub skip_internal_diff_if_external_is_configured : bool , }
};
}

// Generated macro for join_path_segments (function)
macro_rules! Depcrate_mock_traitjoin_path_segments {
() => {
// Module: crate::mock_trait
// Provides: {"join_path_segments"}
// Dependencies: {}
fn join_path_segments (path : & Path , sep : & str) -> String { let mut output = String :: new () ; for segment in & path . segments { if write ! (output , "{}{}" , if output . is_empty () { "" } else { sep } , segment . ident) . is_err () { break ; } ; } output }
};
}

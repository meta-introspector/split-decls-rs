// Generated macro for match_pathsegs (function)
macro_rules! Depcrate_utilmatch_pathsegs {
() => {
// Module: crate::util
// Provides: {"match_pathsegs"}
// Dependencies: {}
# [doc = " Returns true iff the given path matches any of given"] # [doc = " paths specified as string slices."] pub fn match_pathsegs (path : & syn :: Path , against : & [& str]) -> bool { against . iter () . any (| needle | eq_simple_path (needle , path)) }
};
}

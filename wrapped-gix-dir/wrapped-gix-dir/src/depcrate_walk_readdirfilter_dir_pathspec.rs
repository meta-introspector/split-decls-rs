// Generated macro for filter_dir_pathspec (function)
macro_rules! Depcrate_walk_readdirfilter_dir_pathspec {
() => {
// Module: crate::walk::readdir
// Provides: {"filter_dir_pathspec"}
// Dependencies: {}
fn filter_dir_pathspec (current : Option < PathspecMatch >) -> Option < PathspecMatch > { current . filter (| m | { matches ! (m , PathspecMatch :: Always | PathspecMatch :: WildcardMatch | PathspecMatch :: Verbatim) }) }
};
}

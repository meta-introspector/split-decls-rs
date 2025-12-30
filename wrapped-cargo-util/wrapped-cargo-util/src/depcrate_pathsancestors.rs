// Generated macro for ancestors (function)
macro_rules! Depcrate_pathsancestors {
() => {
// Module: crate::paths
// Provides: {"ancestors"}
// Dependencies: {}
# [doc = " Returns an iterator that walks up the directory hierarchy towards the root."] # [doc = ""] # [doc = " Each item is a [`Path`]. It will start with the given path, finishing at"] # [doc = " the root. If the `stop_root_at` parameter is given, it will stop at the"] # [doc = " given path (which will be the last item)."] pub fn ancestors < 'a > (path : & 'a Path , stop_root_at : Option < & Path >) -> PathAncestors < 'a > { PathAncestors :: new (path , stop_root_at) }
};
}

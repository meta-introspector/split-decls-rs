// Generated macro for impl_48 (impl)
macro_rules! Depcrate_pathsimpl_48 {
() => {
// Module: crate::paths
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > Iterator for PathAncestors < 'a > { type Item = & 'a Path ; fn next (& mut self) -> Option < & 'a Path > { if let Some (path) = self . current { self . current = path . parent () ; if let Some (ref stop_at) = self . stop_at { if path == stop_at { self . current = None ; } } Some (path) } else { None } } }
};
}

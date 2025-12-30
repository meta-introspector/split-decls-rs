// Generated macro for HunkCb (type)
macro_rules! Depcrate_diffHunkCb {
() => {
// Module: crate::diff
// Provides: {"HunkCb"}
// Dependencies: {}
pub type HunkCb < 'a > = dyn FnMut (DiffDelta < '_ > , DiffHunk < '_ >) -> bool + 'a ;
};
}

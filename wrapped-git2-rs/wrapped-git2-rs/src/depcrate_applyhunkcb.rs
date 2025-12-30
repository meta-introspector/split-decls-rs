// Generated macro for HunkCB (type)
macro_rules! Depcrate_applyHunkCB {
() => {
// Module: crate::apply
// Provides: {"HunkCB"}
// Dependencies: {}
type HunkCB < 'a > = dyn FnMut (Option < DiffHunk < '_ > >) -> bool + 'a ;
};
}

// Generated macro for LineCb (type)
macro_rules! Depcrate_diffLineCb {
() => {
// Module: crate::diff
// Provides: {"LineCb"}
// Dependencies: {}
pub type LineCb < 'a > = dyn FnMut (DiffDelta < '_ > , Option < DiffHunk < '_ > > , DiffLine < '_ >) -> bool + 'a ;
};
}

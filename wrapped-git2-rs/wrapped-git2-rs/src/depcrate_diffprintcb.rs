// Generated macro for PrintCb (type)
macro_rules! Depcrate_diffPrintCb {
() => {
// Module: crate::diff
// Provides: {"PrintCb"}
// Dependencies: {}
type PrintCb < 'a > = dyn FnMut (DiffDelta < '_ > , Option < DiffHunk < '_ > > , DiffLine < '_ >) -> bool + 'a ;
};
}

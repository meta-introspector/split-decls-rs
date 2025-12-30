// Generated macro for DeltaCB (type)
macro_rules! Depcrate_applyDeltaCB {
() => {
// Module: crate::apply
// Provides: {"DeltaCB"}
// Dependencies: {}
type DeltaCB < 'a > = dyn FnMut (Option < DiffDelta < '_ > >) -> bool + 'a ;
};
}

// Generated macro for BinaryCb (type)
macro_rules! Depcrate_diffBinaryCb {
() => {
// Module: crate::diff
// Provides: {"BinaryCb"}
// Dependencies: {}
pub type BinaryCb < 'a > = dyn FnMut (DiffDelta < '_ > , DiffBinary < '_ >) -> bool + 'a ;
};
}

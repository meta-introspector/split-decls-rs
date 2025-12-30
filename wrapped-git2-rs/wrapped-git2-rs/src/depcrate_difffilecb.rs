// Generated macro for FileCb (type)
macro_rules! Depcrate_diffFileCb {
() => {
// Module: crate::diff
// Provides: {"FileCb"}
// Dependencies: {}
pub type FileCb < 'a > = dyn FnMut (DiffDelta < '_ > , f32) -> bool + 'a ;
};
}

// Generated macro for rescale (function)
macro_rules! Depcrate_saferescale {
() => {
// Module: crate::safe
// Provides: {"rescale"}
// Dependencies: {}
fn rescale (data : & mut [f64] , scale : usize) { let scale = 1. / scale as f64 ; for elm in data { * elm *= scale ; } }
};
}

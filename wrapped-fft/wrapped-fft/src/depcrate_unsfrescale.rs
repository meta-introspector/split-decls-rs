// Generated macro for rescale (function)
macro_rules! Depcrate_unsfrescale {
() => {
// Module: crate::unsf
// Provides: {"rescale"}
// Dependencies: {}
unsafe fn rescale (data : * mut f64 , n : usize) { let scale = 1. / n as f64 ; for i in 0 .. 2 * n { * data . add (i) = * data . add (i) * scale ; } }
};
}

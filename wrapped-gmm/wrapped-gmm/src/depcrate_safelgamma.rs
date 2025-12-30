// Generated macro for lgamma (function)
macro_rules! Depcrate_safelgamma {
() => {
// Module: crate::safe
// Provides: {"lgamma"}
// Dependencies: {}
# [cfg (not (feature = "libm"))] # [inline] fn lgamma (x : f64) -> f64 { unsafe { cmath :: lgamma (x) } }
};
}

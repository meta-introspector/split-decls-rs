// Generated macro for cmath (module)
macro_rules! Depcrate_safecmath {
() => {
// Module: crate::safe
// Provides: {"cmath"}
// Dependencies: {}
# [cfg (not (feature = "libm"))] mod cmath { extern "C" { pub fn lgamma (x : f64) -> f64 ; } }
};
}

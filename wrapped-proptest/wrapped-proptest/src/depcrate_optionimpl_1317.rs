// Generated macro for impl_1317 (impl)
macro_rules! Depcrate_optionimpl_1317 {
() => {
// Module: crate::option
// Provides: {"impl_1317"}
// Dependencies: {}
impl From < f64 > for Probability { # [doc = " Creates a `Probability` from a `f64`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the probability is outside interval `[0.0, 1.0]`."] fn from (prob : f64) -> Self { Probability :: new (prob) } }
};
}

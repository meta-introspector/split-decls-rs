// Generated macro for tests (module)
macro_rules! Depcrate_math_cbrttests {
() => {
// Module: crate::math::cbrt
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn spot_checks () { if ! cfg ! (x86_no_sse) { assert_biteq ! (cbrt (f64 :: from_bits (0xf7f792b28f600000)) , f64 :: from_bits (0xd29ce68655d962f3)) ; } } }
};
}

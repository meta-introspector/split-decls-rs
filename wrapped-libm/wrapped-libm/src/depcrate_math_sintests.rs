// Generated macro for tests (module)
macro_rules! Depcrate_math_sintests {
() => {
// Module: crate::math::sin
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [cfg_attr (x86_no_sse , ignore = "FIXME(i586): possible incorrect rounding")] fn test_near_pi () { let x = f64 :: from_bits (0x400921fb000FD5DD) ; let sx = f64 :: from_bits (0x3ea50d15ced1a4a2) ; assert_eq ! (sin (x) , sx) ; } }
};
}

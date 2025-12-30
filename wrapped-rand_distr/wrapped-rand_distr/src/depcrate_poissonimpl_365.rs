// Generated macro for impl_365 (impl)
macro_rules! Depcrate_poissonimpl_365 {
() => {
// Module: crate::poisson
// Provides: {"impl_365"}
// Dependencies: {}
impl < F > Distribution < F > for KnuthMethod < F > where F : Float + FloatConst , StandardUniform : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let mut result = F :: one () ; let mut p = rng . random :: < F > () ; while p > self . exp_lambda { p = p * rng . random :: < F > () ; result = result + F :: one () ; } result - F :: one () } }
};
}

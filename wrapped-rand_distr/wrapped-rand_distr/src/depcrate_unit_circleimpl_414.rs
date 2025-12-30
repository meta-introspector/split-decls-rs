// Generated macro for impl_414 (impl)
macro_rules! Depcrate_unit_circleimpl_414 {
() => {
// Module: crate::unit_circle
// Provides: {"impl_414"}
// Dependencies: {}
impl < F : Float + SampleUniform > Distribution < [F ; 2] > for UnitCircle { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [F ; 2] { let uniform = Uniform :: new (F :: from (- 1.) . unwrap () , F :: from (1.) . unwrap ()) . unwrap () ; let mut x1 ; let mut x2 ; let mut sum ; loop { x1 = uniform . sample (rng) ; x2 = uniform . sample (rng) ; sum = x1 * x1 + x2 * x2 ; if sum < F :: from (1.) . unwrap () { break ; } } let diff = x1 * x1 - x2 * x2 ; [diff / sum , F :: from (2.) . unwrap () * x1 * x2 / sum] } }
};
}

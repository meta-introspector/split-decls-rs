// Generated macro for impl_408 (impl)
macro_rules! Depcrate_unit_ballimpl_408 {
() => {
// Module: crate::unit_ball
// Provides: {"impl_408"}
// Dependencies: {}
impl < F : Float + SampleUniform > Distribution < [F ; 3] > for UnitBall { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [F ; 3] { let uniform = Uniform :: new (F :: from (- 1.) . unwrap () , F :: from (1.) . unwrap ()) . unwrap () ; let mut x1 ; let mut x2 ; let mut x3 ; loop { x1 = uniform . sample (rng) ; x2 = uniform . sample (rng) ; x3 = uniform . sample (rng) ; if x1 * x1 + x2 * x2 + x3 * x3 <= F :: from (1.) . unwrap () { break ; } } [x1 , x2 , x3] } }
};
}

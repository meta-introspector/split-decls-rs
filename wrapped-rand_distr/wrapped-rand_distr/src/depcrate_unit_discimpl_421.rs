// Generated macro for impl_421 (impl)
macro_rules! Depcrate_unit_discimpl_421 {
() => {
// Module: crate::unit_disc
// Provides: {"impl_421"}
// Dependencies: {}
impl < F : Float + SampleUniform > Distribution < [F ; 2] > for UnitDisc { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [F ; 2] { let uniform = Uniform :: new (F :: from (- 1.) . unwrap () , F :: from (1.) . unwrap ()) . unwrap () ; let mut x1 ; let mut x2 ; loop { x1 = uniform . sample (rng) ; x2 = uniform . sample (rng) ; if x1 * x1 + x2 * x2 <= F :: from (1.) . unwrap () { break ; } } [x1 , x2] } }
};
}

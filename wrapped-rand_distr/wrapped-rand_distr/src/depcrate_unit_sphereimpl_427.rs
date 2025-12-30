// Generated macro for impl_427 (impl)
macro_rules! Depcrate_unit_sphereimpl_427 {
() => {
// Module: crate::unit_sphere
// Provides: {"impl_427"}
// Dependencies: {}
impl < F : Float + SampleUniform > Distribution < [F ; 3] > for UnitSphere { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [F ; 3] { let uniform = Uniform :: new (F :: from (- 1.) . unwrap () , F :: from (1.) . unwrap ()) . unwrap () ; loop { let (x1 , x2) = (uniform . sample (rng) , uniform . sample (rng)) ; let sum = x1 * x1 + x2 * x2 ; if sum >= F :: from (1.) . unwrap () { continue ; } let factor = F :: from (2.) . unwrap () * (F :: one () - sum) . sqrt () ; return [x1 * factor , x2 * factor , F :: from (1.) . unwrap () - F :: from (2.) . unwrap () * sum ,] ; } } }
};
}

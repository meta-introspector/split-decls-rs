// Generated macro for impl_362 (impl)
macro_rules! Depcrate_poissonimpl_362 {
() => {
// Module: crate::poisson
// Provides: {"impl_362"}
// Dependencies: {}
impl < F : Float + FloatConst > RejectionMethod < F > { pub (crate) fn new (lambda : F) -> Self { let b1 = F :: from (1.0 / 24.0) . unwrap () / lambda ; let b2 = F :: from (0.3) . unwrap () * b1 * b1 ; let c3 = F :: from (1.0 / 7.0) . unwrap () * b1 * b2 ; let c2 = b2 - F :: from (15) . unwrap () * c3 ; let c1 = b1 - F :: from (6) . unwrap () * b2 + F :: from (45) . unwrap () * c3 ; let c0 = F :: one () - b1 + F :: from (3) . unwrap () * b2 - F :: from (15) . unwrap () * c3 ; RejectionMethod { lambda , s : lambda . sqrt () , d : F :: from (6.0) . unwrap () * lambda . powi (2) , l : (lambda - F :: from (1.1484) . unwrap ()) . floor () , c : F :: from (0.1069) . unwrap () / lambda , c0 , c1 , c2 , c3 , omega : F :: one () / (F :: from (2) . unwrap () * F :: PI ()) . sqrt () / lambda . sqrt () , } } }
};
}

// Generated macro for impl_236 (impl)
macro_rules! Depcrate_gammaimpl_236 {
() => {
// Module: crate::gamma
// Provides: {"impl_236"}
// Dependencies: {}
impl < F > Distribution < F > for GammaLargeShape < F > where F : Float , StandardNormal : Distribution < F > , Open01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { loop { let x : F = rng . sample (StandardNormal) ; let v_cbrt = F :: one () + self . c * x ; if v_cbrt <= F :: zero () { continue ; } let v = v_cbrt * v_cbrt * v_cbrt ; let u : F = rng . sample (Open01) ; let x_sqr = x * x ; if u < F :: one () - F :: from (0.0331) . unwrap () * x_sqr * x_sqr || u . ln () < F :: from (0.5) . unwrap () * x_sqr + self . d * (F :: one () - v + v . ln ()) { return self . d * v * self . scale ; } } } }
};
}

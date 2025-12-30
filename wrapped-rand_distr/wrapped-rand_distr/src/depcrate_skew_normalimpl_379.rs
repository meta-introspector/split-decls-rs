// Generated macro for impl_379 (impl)
macro_rules! Depcrate_skew_normalimpl_379 {
() => {
// Module: crate::skew_normal
// Provides: {"impl_379"}
// Dependencies: {}
impl < F > Distribution < F > for SkewNormal < F > where F : Float , StandardNormal : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let linear_map = | x : F | -> F { x * self . scale + self . location } ; let u_1 : F = rng . sample (StandardNormal) ; if self . shape == F :: zero () { linear_map (u_1) } else { let u_2 = rng . sample (StandardNormal) ; let (u , v) = (u_1 . max (u_2) , u_1 . min (u_2)) ; if self . shape == - F :: one () { linear_map (v) } else if self . shape == F :: one () { linear_map (u) } else { let normalized = ((F :: one () + self . shape) * u + (F :: one () - self . shape) * v) / ((F :: one () + self . shape * self . shape) . sqrt () * F :: from (core :: f64 :: consts :: SQRT_2) . unwrap ()) ; linear_map (normalized) } } } }
};
}

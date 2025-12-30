// Generated macro for impl_459 (impl)
macro_rules! Depcrate_zetaimpl_459 {
() => {
// Module: crate::zeta
// Provides: {"impl_459"}
// Dependencies: {}
impl < F > Distribution < F > for Zeta < F > where F : Float , StandardUniform : Distribution < F > , OpenClosed01 : Distribution < F > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { loop { let u = rng . sample (OpenClosed01) ; let x = u . powf (- F :: one () / self . s_minus_1) . floor () ; debug_assert ! (x >= F :: one ()) ; if x . is_infinite () { return x ; } let t = (F :: one () + F :: one () / x) . powf (self . s_minus_1) ; let v = rng . sample (StandardUniform) ; if v * x * (t - F :: one ()) * self . b <= t * (self . b - F :: one ()) { return x ; } } } }
};
}

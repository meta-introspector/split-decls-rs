// Generated macro for impl_447 (impl)
macro_rules! Depcrate_weibullimpl_447 {
() => {
// Module: crate::weibull
// Provides: {"impl_447"}
// Dependencies: {}
impl < F > Distribution < F > for Weibull < F > where F : Float , OpenClosed01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let x : F = rng . sample (OpenClosed01) ; self . scale * (- x . ln ()) . powf (self . inv_shape) } }
};
}

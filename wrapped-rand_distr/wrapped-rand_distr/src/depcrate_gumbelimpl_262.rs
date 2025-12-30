// Generated macro for impl_262 (impl)
macro_rules! Depcrate_gumbelimpl_262 {
() => {
// Module: crate::gumbel
// Provides: {"impl_262"}
// Dependencies: {}
impl < F > Distribution < F > for Gumbel < F > where F : Float , OpenClosed01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let x : F = rng . sample (OpenClosed01) ; self . location - self . scale * (- x . ln ()) . ln () } }
};
}

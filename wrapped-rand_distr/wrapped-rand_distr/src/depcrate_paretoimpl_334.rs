// Generated macro for impl_334 (impl)
macro_rules! Depcrate_paretoimpl_334 {
() => {
// Module: crate::pareto
// Provides: {"impl_334"}
// Dependencies: {}
impl < F > Distribution < F > for Pareto < F > where F : Float , OpenClosed01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let u : F = OpenClosed01 . sample (rng) ; self . scale * u . powf (self . inv_neg_shape) } }
};
}

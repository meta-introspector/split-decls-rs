// Generated macro for impl_235 (impl)
macro_rules! Depcrate_gammaimpl_235 {
() => {
// Module: crate::gamma
// Provides: {"impl_235"}
// Dependencies: {}
impl < F > Distribution < F > for GammaSmallShape < F > where F : Float , StandardNormal : Distribution < F > , Open01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let u : F = rng . sample (Open01) ; self . large_shape . sample (rng) * u . powf (self . inv_shape) } }
};
}

// Generated macro for impl_322 (impl)
macro_rules! Depcrate_normal_inverse_gaussianimpl_322 {
() => {
// Module: crate::normal_inverse_gaussian
// Provides: {"impl_322"}
// Dependencies: {}
impl < F > Distribution < F > for NormalInverseGaussian < F > where F : Float , StandardNormal : Distribution < F > , StandardUniform : Distribution < F > , { fn sample < R > (& self , rng : & mut R) -> F where R : Rng + ? Sized , { let inv_gauss = rng . sample (self . inverse_gaussian) ; self . beta * inv_gauss + inv_gauss . sqrt () * rng . sample (StandardNormal) } }
};
}

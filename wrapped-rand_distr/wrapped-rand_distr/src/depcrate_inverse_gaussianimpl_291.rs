// Generated macro for impl_291 (impl)
macro_rules! Depcrate_inverse_gaussianimpl_291 {
() => {
// Module: crate::inverse_gaussian
// Provides: {"impl_291"}
// Dependencies: {}
impl < F > Distribution < F > for InverseGaussian < F > where F : Float , StandardNormal : Distribution < F > , StandardUniform : Distribution < F > , { # [allow (clippy :: many_single_char_names)] fn sample < R > (& self , rng : & mut R) -> F where R : Rng + ? Sized , { let mu = self . mean ; let l = self . shape ; let v : F = rng . sample (StandardNormal) ; let y = mu * v * v ; let mu_2l = mu / (F :: from (2.) . unwrap () * l) ; let x = mu + mu_2l * (y - (F :: from (4.) . unwrap () * l * y + y * y) . sqrt ()) ; let u : F = rng . random () ; if u <= mu / (mu + x) { return x ; } mu * mu / x } }
};
}

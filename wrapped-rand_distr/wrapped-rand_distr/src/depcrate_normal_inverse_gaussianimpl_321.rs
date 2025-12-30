// Generated macro for impl_321 (impl)
macro_rules! Depcrate_normal_inverse_gaussianimpl_321 {
() => {
// Module: crate::normal_inverse_gaussian
// Provides: {"impl_321"}
// Dependencies: {}
impl < F > NormalInverseGaussian < F > where F : Float , StandardNormal : Distribution < F > , StandardUniform : Distribution < F > , { # [doc = " Construct a new `NormalInverseGaussian` distribution with the given alpha (tail heaviness) and"] # [doc = " beta (asymmetry) parameters."] pub fn new (alpha : F , beta : F) -> Result < NormalInverseGaussian < F > , Error > { if ! (alpha > F :: zero ()) { return Err (Error :: AlphaNegativeOrNull) ; } if ! (beta . abs () < alpha) { return Err (Error :: AbsoluteBetaNotLessThanAlpha) ; } let gamma = (alpha * alpha - beta * beta) . sqrt () ; let mu = F :: one () / gamma ; let inverse_gaussian = InverseGaussian :: new (mu , F :: one ()) . unwrap () ; Ok (Self { beta , inverse_gaussian , }) } }
};
}

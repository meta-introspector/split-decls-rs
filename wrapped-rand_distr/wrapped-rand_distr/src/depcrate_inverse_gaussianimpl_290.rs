// Generated macro for impl_290 (impl)
macro_rules! Depcrate_inverse_gaussianimpl_290 {
() => {
// Module: crate::inverse_gaussian
// Provides: {"impl_290"}
// Dependencies: {}
impl < F > InverseGaussian < F > where F : Float , StandardNormal : Distribution < F > , StandardUniform : Distribution < F > , { # [doc = " Construct a new `InverseGaussian` distribution with the given mean and"] # [doc = " shape."] pub fn new (mean : F , shape : F) -> Result < InverseGaussian < F > , Error > { let zero = F :: zero () ; if ! (mean > zero) { return Err (Error :: MeanNegativeOrNull) ; } if ! (shape > zero) { return Err (Error :: ShapeNegativeOrNull) ; } Ok (Self { mean , shape }) } }
};
}

// Generated macro for impl_202 (impl)
macro_rules! Depcrate_fisher_fimpl_202 {
() => {
// Module: crate::fisher_f
// Provides: {"impl_202"}
// Dependencies: {}
impl < F > FisherF < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Create a new `FisherF` distribution, with the given parameter."] pub fn new (m : F , n : F) -> Result < FisherF < F > , Error > { let zero = F :: zero () ; if ! (m > zero) { return Err (Error :: MTooSmall) ; } if ! (n > zero) { return Err (Error :: NTooSmall) ; } Ok (FisherF { numer : ChiSquared :: new (m) . unwrap () , denom : ChiSquared :: new (n) . unwrap () , dof_ratio : n / m , }) } }
};
}

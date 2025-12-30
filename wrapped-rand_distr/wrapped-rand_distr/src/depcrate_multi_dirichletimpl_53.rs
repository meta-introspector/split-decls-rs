// Generated macro for impl_53 (impl)
macro_rules! Depcrate_multi_dirichletimpl_53 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_53"}
// Dependencies: {}
impl < F > DirichletFromBeta < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Construct a new `DirichletFromBeta` with the given parameters `alpha`."] # [doc = ""] # [doc = " This function is part of a private implementation detail."] # [doc = " It assumes that the input is correct, so no validation of alpha is done."] # [inline] fn new (alpha : & [F]) -> Result < DirichletFromBeta < F > , DirichletFromBetaError > { let n = alpha . len () ; let mut alpha_rev_csum = vec ! [alpha [n - 1] ; n - 1] ; for k in 0 .. (n - 2) { alpha_rev_csum [n - 3 - k] = alpha_rev_csum [n - 2 - k] + alpha [n - 2 - k] ; } let mut beta_dists = Vec :: new () ; for (& a , & b) in alpha [.. (n - 1)] . iter () . zip (alpha_rev_csum . iter ()) { let dist = Beta :: new (a , b) . map_err (| _ | DirichletFromBetaError :: BetaNewFailed) ? ; beta_dists . push (dist) ; } Ok (DirichletFromBeta { samplers : beta_dists . into_boxed_slice () , }) } }
};
}

// Generated macro for impl_49 (impl)
macro_rules! Depcrate_multi_dirichletimpl_49 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_49"}
// Dependencies: {}
impl < F > DirichletFromGamma < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Construct a new `DirichletFromGamma` with the given parameters `alpha`."] # [doc = ""] # [doc = " This function is part of a private implementation detail."] # [doc = " It assumes that the input is correct, so no validation of alpha is done."] # [inline] fn new (alpha : & [F]) -> Result < DirichletFromGamma < F > , DirichletFromGammaError > { let mut gamma_dists = Vec :: new () ; for a in alpha { let dist = Gamma :: new (* a , F :: one ()) . map_err (| _ | DirichletFromGammaError :: GammmaNewFailed) ? ; gamma_dists . push (dist) ; } Ok (DirichletFromGamma { samplers : gamma_dists , }) } }
};
}

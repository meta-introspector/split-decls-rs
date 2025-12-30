// Generated macro for impl_60 (impl)
macro_rules! Depcrate_multi_dirichletimpl_60 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_60"}
// Dependencies: {}
impl < F > Dirichlet < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Construct a new `Dirichlet` with the given alpha parameter `alpha`."] # [doc = ""] # [doc = " Requires `alpha.len() >= 2`, and each value in `alpha` must be positive,"] # [doc = " finite and not subnormal."] # [inline] pub fn new (alpha : & [F]) -> Result < Dirichlet < F > , Error > { if alpha . len () < 2 { return Err (Error :: AlphaTooShort) ; } for & ai in alpha . iter () { if ! (ai > F :: zero ()) { return Err (Error :: AlphaTooSmall) ; } if ai . is_infinite () { return Err (Error :: AlphaInfinite) ; } if ! ai . is_normal () { return Err (Error :: AlphaSubnormal) ; } } if alpha . iter () . all (| & x | x <= NumCast :: from (0.1) . unwrap ()) { let dist = DirichletFromBeta :: new (alpha) . map_err (| _ | Error :: FailedToCreateBeta) ? ; Ok (Dirichlet { repr : DirichletRepr :: FromBeta (dist) , }) } else { let dist = DirichletFromGamma :: new (alpha) . map_err (| _ | Error :: FailedToCreateGamma) ? ; Ok (Dirichlet { repr : DirichletRepr :: FromGamma (dist) , }) } } }
};
}

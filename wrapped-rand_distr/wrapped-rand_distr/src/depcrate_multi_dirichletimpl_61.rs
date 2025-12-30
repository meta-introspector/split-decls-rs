// Generated macro for impl_61 (impl)
macro_rules! Depcrate_multi_dirichletimpl_61 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_61"}
// Dependencies: {}
impl < F > MultiDistribution < F > for Dirichlet < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [inline] fn sample_len (& self) -> usize { match & self . repr { DirichletRepr :: FromGamma (dirichlet) => dirichlet . sample_len () , DirichletRepr :: FromBeta (dirichlet) => dirichlet . sample_len () , } } fn sample_to_slice < R : Rng + ? Sized > (& self , rng : & mut R , output : & mut [F]) { match & self . repr { DirichletRepr :: FromGamma (dirichlet) => dirichlet . sample_to_slice (rng , output) , DirichletRepr :: FromBeta (dirichlet) => dirichlet . sample_to_slice (rng , output) , } } }
};
}

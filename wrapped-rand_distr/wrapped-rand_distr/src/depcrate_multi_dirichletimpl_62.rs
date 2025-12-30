// Generated macro for impl_62 (impl)
macro_rules! Depcrate_multi_dirichletimpl_62 {
() => {
// Module: crate::multi::dirichlet
// Provides: {"impl_62"}
// Dependencies: {}
impl < F > Distribution < Vec < F > > for Dirichlet < F > where F : Float + Default , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { distribution_impl ! (F) ; }
};
}

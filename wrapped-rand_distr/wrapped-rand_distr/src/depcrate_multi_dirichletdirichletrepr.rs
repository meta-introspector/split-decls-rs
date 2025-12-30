// Generated macro for DirichletRepr (enum)
macro_rules! Depcrate_multi_dirichletDirichletRepr {
() => {
// Module: crate::multi::dirichlet
// Provides: {"DirichletRepr"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , serde_as)] enum DirichletRepr < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Dirichlet distribution that generates samples using the Gamma distribution."] FromGamma (DirichletFromGamma < F >) , # [doc = " Dirichlet distribution that generates samples using the Beta distribution."] FromBeta (DirichletFromBeta < F >) , }
};
}

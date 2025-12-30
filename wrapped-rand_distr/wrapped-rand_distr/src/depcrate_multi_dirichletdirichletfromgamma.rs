// Generated macro for DirichletFromGamma (struct)
macro_rules! Depcrate_multi_dirichletDirichletFromGamma {
() => {
// Module: crate::multi::dirichlet
// Provides: {"DirichletFromGamma"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , serde_as)] struct DirichletFromGamma < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { samplers : Vec < Gamma < F > > , }
};
}

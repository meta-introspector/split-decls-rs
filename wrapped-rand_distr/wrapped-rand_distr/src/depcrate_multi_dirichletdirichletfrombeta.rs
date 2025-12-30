// Generated macro for DirichletFromBeta (struct)
macro_rules! Depcrate_multi_dirichletDirichletFromBeta {
() => {
// Module: crate::multi::dirichlet
// Provides: {"DirichletFromBeta"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] struct DirichletFromBeta < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { samplers : Box < [Beta < F >] > , }
};
}

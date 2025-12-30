// Generated macro for ChiSquaredRepr (enum)
macro_rules! Depcrate_chi_squaredChiSquaredRepr {
() => {
// Module: crate::chi_squared
// Provides: {"ChiSquaredRepr"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] enum ChiSquaredRepr < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { DoFExactlyOne , DoFAnythingElse (Gamma < F >) , }
};
}

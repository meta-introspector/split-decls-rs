// Generated macro for GammaRepr (enum)
macro_rules! Depcrate_gammaGammaRepr {
() => {
// Module: crate::gamma
// Provides: {"GammaRepr"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] enum GammaRepr < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { Large (GammaLargeShape < F >) , One (Exp < F >) , Small (GammaSmallShape < F >) , }
};
}

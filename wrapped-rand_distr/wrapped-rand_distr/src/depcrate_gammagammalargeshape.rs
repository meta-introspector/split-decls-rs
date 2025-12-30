// Generated macro for GammaLargeShape (struct)
macro_rules! Depcrate_gammaGammaLargeShape {
() => {
// Module: crate::gamma
// Provides: {"GammaLargeShape"}
// Dependencies: {}
# [doc = " Gamma distribution where the shape parameter is larger than 1."] # [doc = ""] # [doc = " See `Gamma` for sampling from a Gamma distribution with general"] # [doc = " shape parameters."] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] struct GammaLargeShape < F > where F : Float , StandardNormal : Distribution < F > , Open01 : Distribution < F > , { scale : F , c : F , d : F , }
};
}

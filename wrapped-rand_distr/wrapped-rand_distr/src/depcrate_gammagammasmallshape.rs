// Generated macro for GammaSmallShape (struct)
macro_rules! Depcrate_gammaGammaSmallShape {
() => {
// Module: crate::gamma
// Provides: {"GammaSmallShape"}
// Dependencies: {}
# [doc = " Gamma distribution where the shape parameter is less than 1."] # [doc = ""] # [doc = " Note, samples from this require a compulsory floating-point `pow`"] # [doc = " call, which makes it significantly slower than sampling from a"] # [doc = " gamma distribution where the shape parameter is greater than or"] # [doc = " equal to 1."] # [doc = ""] # [doc = " See `Gamma` for sampling from a Gamma distribution with general"] # [doc = " shape parameters."] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] struct GammaSmallShape < F > where F : Float , StandardNormal : Distribution < F > , Open01 : Distribution < F > , { inv_shape : F , large_shape : GammaLargeShape < F > , }
};
}

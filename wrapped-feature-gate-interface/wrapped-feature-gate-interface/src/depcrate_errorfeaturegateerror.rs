// Generated macro for FeatureGateError (enum)
macro_rules! Depcrate_errorFeatureGateError {
() => {
// Module: crate::error
// Provides: {"FeatureGateError"}
// Dependencies: {}
# [doc = " Program error types."] # [cfg_attr (test , derive (strum_macros :: FromRepr , strum_macros :: EnumIter))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Clone , Debug , PartialEq , Eq)] # [repr (u32)] pub enum FeatureGateError { # [doc = " Feature already activated"] FeatureAlreadyActivated , }
};
}

// Generated macro for AuthType (enum)
macro_rules! Depcrate_providersAuthType {
() => {
// Module: crate::providers
// Provides: {"AuthType"}
// Dependencies: {}
# [doc = " Authentication type for providers"] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq)] # [serde (rename_all = "lowercase")] pub enum AuthType { # [doc = " API key authentication"] ApiKey , # [doc = " OAuth 2.0 authentication"] OAuth , }
};
}

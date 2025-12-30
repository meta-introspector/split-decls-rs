// Generated macro for ModelMapping (struct)
macro_rules! Depcrate_configModelMapping {
() => {
// Module: crate::config
// Provides: {"ModelMapping"}
// Dependencies: {}
# [doc = " Model mapping to a specific provider"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct ModelMapping { # [doc = " Priority for this mapping (1 = highest priority)"] pub priority : u32 , # [doc = " Provider name"] pub provider : String , # [doc = " Actual model name to use with the provider"] pub actual_model : String , }
};
}

// Generated macro for ModelConfig (struct)
macro_rules! Depcrate_configModelConfig {
() => {
// Module: crate::config
// Provides: {"ModelConfig"}
// Dependencies: {}
# [doc = " Model configuration with 1:N provider mappings"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct ModelConfig { # [doc = " External model name (used in API requests)"] pub name : String , # [doc = " List of provider mappings with priorities (fallback support)"] pub mappings : Vec < ModelMapping > , }
};
}

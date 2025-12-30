// Generated macro for AppConfig (struct)
macro_rules! Depcrate_configAppConfig {
() => {
// Module: crate::config
// Provides: {"AppConfig"}
// Dependencies: {}
# [doc = " Application configuration"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct AppConfig { # [serde (default)] pub server : ServerConfig , pub router : RouterConfig , # [serde (default)] pub providers : Vec < ProviderConfig > , # [serde (default)] pub models : Vec < ModelConfig > , }
};
}

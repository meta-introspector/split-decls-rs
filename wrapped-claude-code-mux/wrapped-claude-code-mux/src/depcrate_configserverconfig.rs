// Generated macro for ServerConfig (struct)
macro_rules! Depcrate_configServerConfig {
() => {
// Module: crate::config
// Provides: {"ServerConfig"}
// Dependencies: {}
# [doc = " Server configuration"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct ServerConfig { # [serde (default = "default_port")] pub port : u16 , # [serde (default = "default_host")] pub host : String , pub api_key : Option < String > , # [serde (default = "default_log_level")] pub log_level : String , # [serde (default)] pub timeouts : TimeoutConfig , }
};
}

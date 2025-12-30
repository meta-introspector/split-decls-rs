// Generated macro for TimeoutConfig (struct)
macro_rules! Depcrate_configTimeoutConfig {
() => {
// Module: crate::config
// Provides: {"TimeoutConfig"}
// Dependencies: {}
# [doc = " Timeout configuration"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct TimeoutConfig { # [serde (default = "default_api_timeout")] pub api_timeout_ms : u64 , # [serde (default = "default_connect_timeout")] pub connect_timeout_ms : u64 , }
};
}

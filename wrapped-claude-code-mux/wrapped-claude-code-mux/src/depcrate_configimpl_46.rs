// Generated macro for impl_46 (impl)
macro_rules! Depcrate_configimpl_46 {
() => {
// Module: crate::config
// Provides: {"impl_46"}
// Dependencies: {}
impl Default for ServerConfig { fn default () -> Self { Self { port : default_port () , host : default_host () , api_key : None , log_level : default_log_level () , timeouts : TimeoutConfig :: default () , } } }
};
}

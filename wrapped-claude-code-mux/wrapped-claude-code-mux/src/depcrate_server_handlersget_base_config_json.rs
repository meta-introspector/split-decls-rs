// Generated macro for get_base_config_json (function)
macro_rules! Depcrate_server_handlersget_base_config_json {
() => {
// Module: crate::server::handlers
// Provides: {"get_base_config_json"}
// Dependencies: {}
# [doc = " Creates the base configuration JSON value."] fn get_base_config_json (config : & AppConfig) -> serde_json :: Value { serde_json :: json ! ({ "server" : { "host" : config . server . host . clone () , "port" : config . server . port , } , "router" : { "default" : config . router . default , "background" : config . router . background , "think" : config . router . think , "websearch" : config . router . websearch , } }) }
};
}

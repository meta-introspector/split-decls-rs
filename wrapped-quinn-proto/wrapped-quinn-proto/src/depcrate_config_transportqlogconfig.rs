// Generated macro for QlogConfig (struct)
macro_rules! Depcrate_config_transportQlogConfig {
() => {
// Module: crate::config::transport
// Provides: {"QlogConfig"}
// Dependencies: {}
# [doc = " Configuration for qlog trace logging"] # [cfg (feature = "qlog")] pub struct QlogConfig { writer : Option < Box < dyn io :: Write + Send + Sync > > , title : Option < String > , description : Option < String > , start_time : Instant , }
};
}

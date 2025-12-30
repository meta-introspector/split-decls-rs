// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Config for debouncer-mini"] # [doc = " ```rust"] # [doc = " # use std::time::Duration;"] # [doc = " use notify_debouncer_mini::Config;"] # [doc = " let backend_config = notify::Config::default();"] # [doc = ""] # [doc = " let config = Config::default().with_timeout(Duration::from_secs(1)).with_batch_mode(true)"] # [doc = "     .with_notify_config(backend_config);"] # [doc = " ```"] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct Config { timeout : Duration , batch_mode : bool , notify_config : notify :: Config , }
};
}

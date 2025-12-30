// Generated macro for Config (struct)
macro_rules! Depcrate_configConfig {
() => {
// Module: crate::config
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Watcher Backend configuration"] # [doc = ""] # [doc = " This contains multiple settings that may relate to only one specific backend,"] # [doc = " such as to correctly configure each backend regardless of what is selected during runtime."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::time::Duration;"] # [doc = " # use notify::Config;"] # [doc = " let config = Config::default()"] # [doc = "     .with_poll_interval(Duration::from_secs(2))"] # [doc = "     .with_compare_contents(true);"] # [doc = " ```"] # [doc = ""] # [doc = " Some options can be changed during runtime, others have to be set when creating the watcher backend."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] pub struct Config { # [doc = " See [Config::with_poll_interval]"] poll_interval : Option < Duration > , # [doc = " See [Config::with_compare_contents]"] compare_contents : bool , follow_symlinks : bool , }
};
}

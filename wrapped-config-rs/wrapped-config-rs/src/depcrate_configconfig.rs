// Generated macro for Config (struct)
macro_rules! Depcrate_configConfig {
() => {
// Module: crate::config
// Provides: {"Config"}
// Dependencies: {}
# [doc = " A prioritized configuration repository."] # [doc = ""] # [doc = " It maintains a set of configuration sources, fetches values to populate those, and provides"] # [doc = " them according to the source's priority."] # [derive (Clone , Debug)] pub struct Config { defaults : Map < path :: Expression , Value > , overrides : Map < path :: Expression , Value > , sources : Vec < Box < dyn Source + Send + Sync > > , # [doc = " Root of the cached configuration."] pub cache : Value , }
};
}

// Generated macro for ModuleConfig (struct)
macro_rules! Depcrate_configModuleConfig {
() => {
// Module: crate::config
// Provides: {"ModuleConfig"}
// Dependencies: {}
# [derive (Deserialize , Debug , Default , Clone , PartialEq , Eq)] # [serde (deny_unknown_fields)] pub struct ModuleConfig { # [serde (default)] pub skipped : bool , # [serde (default)] pub module : HashMap < String , ModuleConfig > , }
};
}
